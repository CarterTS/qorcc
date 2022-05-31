use std::collections::HashMap;

use crate::compiler::Compiler;
use crate::preprocessor::PreprocessorError;
use crate::tokenizer::Token;
use crate::errors::*;
use crate::tokenizer::TokenType;

/// Macro Replacements
#[derive(Debug, Clone)]
pub enum MacroReplacements
{
    None,
    DirectReplacement(Vec<Token>),
    FunctionLikeReplacement(Vec<Token>, Vec<Token>)
}

/// Preprocessor Context
pub struct PreprocessorContext<'a, 'b>
{
    compiler: &'a mut Compiler<'b>,
    defines: HashMap<String, MacroReplacements>,
    filename_stack: Vec<String>,
    if_stack: Vec<bool>
}

impl<'a, 'b> PreprocessorContext<'a, 'b>
{
    pub fn with_compiler(compiler: &'a mut Compiler<'b>) -> Self
    {
        Self
        {
            compiler,
            defines: HashMap::new(),
            filename_stack: Vec::new(),
            if_stack: Vec::new()
        }
    }

    /// Return true if the value is defined
    pub fn is_defined(&self, identifier: &str) -> bool
    {
        self.defines.contains_key(identifier)
    }


    /// Undefine an identifier
    pub fn undefine(&mut self, identifier: &str)
    {
        if self.defines.remove(identifier).is_none()
        {
            warn!("Undefining {}, which is not defined", identifier);
        }
    }

    /// Find the filename in the given context
    pub fn search_include_paths(&self, filename: &str) -> CompilerResult<String>
    {
        if let Some(Some(last_path)) = self.filename_stack.last().map(|v| std::path::Path::new(v).parent())
        {
            let p = std::path::Path::new(last_path.to_str().unwrap());
            let this_path = std::path::Path::join(p, filename);

            if this_path.exists()
            {
                return Ok(this_path.to_str().unwrap().to_string());
            }
        }

        Ok(filename.to_string())
    }

    /// Include a file
    pub fn include_file(&mut self, filename: &str) -> CompilerResult<Vec<Token>>
    {
        let path = self.search_include_paths(filename)?;
        
        let v = self.preprocess(&path)?;

        // We need to make sure that the end of file token isn't passed along
        let final_tokens = v.iter().map_while(|v| if !v.is_eof() { Some(v.clone()) } else { None }).collect::<Vec<Token>>();

        Ok(final_tokens)
    }

    /// Preprocess a file into a sequence of tokens, and repeat until no transformation is performed
    pub fn preprocess(&mut self, filename: &str) -> CompilerResult<Vec<Token>>
    {
        trace!("Preprocessing file {}", filename);

        // Make sure the file is tokenized
        self.compiler.get_file_manager(filename)?.tokenize()?;

        // Get the FileManager for that file
        let file = self.compiler.get_file_manager(filename)?.clone();
        self.filename_stack.push(filename.to_string());

        let result = self.preprocess_tokens(file.iter())?;

        if self.filename_stack.pop() != Some(filename.to_string())
        {
            unreachable!()
        }

        Ok(result)
    }

    pub fn preprocess_tokens<'x, I: Iterator<Item = &'x Token>>(&mut self, iterator: I) -> CompilerResult<Vec<Token>>
    {
        let mut result = Vec::new();

        let mut peekable_iter = iterator.peekable();

        while let Some(mut peeked_next) = peekable_iter.peek()
        {
            let current_line = peeked_next.location.line;
            if !self.if_stack.iter().all(|v| *v)
            {
                loop
                {
                    let next = PreprocessorError::prevent_eof(peekable_iter.peek().map(|v| *v))?;

                    if next.token_type == TokenType::PreprocessorDirective(String::from("#ifdef")) ||
                       next.token_type == TokenType::PreprocessorDirective(String::from("#ifndef")) ||
                       next.token_type == TokenType::PreprocessorDirective(String::from("#else")) ||
                       next.token_type == TokenType::PreprocessorDirective(String::from("#endif"))
                    {
                        break;
                    }

                    peekable_iter.next();
                }

                peeked_next = peekable_iter.peek().unwrap();
            }

            if let TokenType::PreprocessorDirective(directive) = &peeked_next.token_type
            {
                match directive.as_str()
                {
                    "#define" =>
                    {
                        // Step to the next symbol
                        peekable_iter.next();

                        // Get the identifier name
                        let identifier = PreprocessorError::expect_identifier(peekable_iter.next())?.code_styled();

                        // Determine the sequence of tokens which this expands to by finding the tokens remaining on the line
                        let mut macro_tokens = Vec::new();
                        while let Some(t) = peekable_iter.peek()
                        {
                            if t.is_eof() { break; }
                            if t.location.line == current_line
                            {
                                macro_tokens.push(peekable_iter.next().unwrap().clone());
                            }
                            else
                            {
                                break;
                            }
                        }

                        // If there are no tokens remaining in the line, we are just defining the macro
                        if macro_tokens.len() == 0
                        {
                            self.register_macro_empty(identifier);
                        }
                        // If the sequence begins with a left paren, then we are starting a function like macro
                        else if TokenType::Symbol(String::from("(")) == macro_tokens[0].token_type
                        {
                            let mut peekable_iter = macro_tokens[1..].iter().peekable();

                            let mut arguments = Vec::new();

                            loop
                            {
                                // Take the next token, which should be an identifier, and add it to the list of arguments
                                arguments.push(PreprocessorError::expect_identifier(peekable_iter.next())?);

                                // Now we have a choice
                                let next_token = PreprocessorError::prevent_eof(peekable_iter.next())?;
                                // If the next symbol is a close paren, break out and use the remainder of the iterator as the pattern
                                if TokenType::Symbol(String::from(")")) == next_token.token_type
                                {
                                    break;
                                }
                                // Otherwise, it must be a comma, in which case we will loop back to the top
                                else
                                {
                                    PreprocessorError::expect_symbol(Some(&next_token.clone()), ",")?;
                                }
                            }

                            let mut pattern = Vec::new();

                            for token in peekable_iter
                            {
                                pattern.push(token.clone());
                            }

                            self.register_function_like_macro(identifier, arguments, pattern);
                        }
                        // Otherwise, we will register the sequence of tokens as the macro
                        else
                        {
                            self.register_direct_replace_macro(identifier, macro_tokens);
                        }
                    },
                    "#include" =>
                    {
                        // Step to the next symbol
                        peekable_iter.next();

                        // Next there should be a string literal with the name of the imported file
                        let filename = PreprocessorError::expect_string_literal(peekable_iter.next())?;

                        // Include the file by adding it to the file processing stack
                        if let TokenType::StringLiteral(filename) = filename.token_type
                        {
                            result.append(&mut self.include_file(&filename)?);
                        }
                        else
                        {
                            unreachable!()
                        }
                    },
                    "#undef" =>
                    {
                        // Step to the next symbol
                        peekable_iter.next();

                        // Get the identifier name
                        let identifier = PreprocessorError::expect_identifier(peekable_iter.next())?.code_styled();

                        self.undefine(&identifier);

                    },
                    "#ifdef" =>
                    {
                        // Step to the next symbol
                        peekable_iter.next();

                        // Get the identifier name
                        let identifier = PreprocessorError::expect_identifier(peekable_iter.next())?.code_styled();
                    
                        // Push the identifier being defined to the if stack
                        self.if_stack.push(self.is_defined(&identifier));
                    },
                    "#ifndef" =>
                    {
                        // Step to the next symbol
                        peekable_iter.next();

                        // Get the identifier name
                        let identifier = PreprocessorError::expect_identifier(peekable_iter.next())?.code_styled();

                        // Push the inverse of the identifier being defined to the if stack
                        self.if_stack.push(!self.is_defined(&identifier));
                    },
                    "#else" =>
                    {
                        // Step to the next symbol
                        let token = peekable_iter.next().unwrap();

                        if let Some(last_if) = self.if_stack.pop()
                        {
                            self.if_stack.push(!last_if);
                        }
                        else
                        {
                            return Err(PreprocessorError::syntax_error(format!("else directive cannot be used outside of a preprocessor if statement"), &token).into());
                        }
                    },
                    "#endif" =>
                    {
                        // Step to the next symbol
                        let token = peekable_iter.next().unwrap();

                        if self.if_stack.pop().is_none()
                        {
                            return Err(PreprocessorError::syntax_error(format!("Unexpected endif directive"), &token).into());
                        }
                    },
                    _ => {return Err(PreprocessorError::syntax_error(format!("Unknown directive {}", directive),  &peeked_next).into())}
                }
                
                continue;
            }
            else if let TokenType::Identifier(identifier) = &peeked_next.token_type
            {
                if let Some(expand_to) = self.defines.get(identifier)
                {
                    let orig_location = peekable_iter.next().unwrap().get_original();

                    match expand_to
                    {
                        MacroReplacements::DirectReplacement(tokens) => 
                        {
                            let mut new_tokens = tokens.clone();

                            for t in &mut new_tokens
                            {
                                t.original_location = Some(orig_location.clone());
                            }

                            result.append(&mut new_tokens);
                        },
                        MacroReplacements::FunctionLikeReplacement(_arguments, _tokens) =>
                        {
                            todo!()
                        },
                        _ => {}
                    }
                    continue;
                }
            }

            result.push(peekable_iter.next().unwrap().clone());
        }

        Ok(result)
    }

    pub fn register_macro_empty(&mut self, name: String)
    {
        self.defines.insert(name, MacroReplacements::None);
    }

    pub fn register_direct_replace_macro(&mut self, name: String, tokens: Vec<Token>)
    {
        self.defines.insert(name, MacroReplacements::DirectReplacement(tokens));
    }

    pub fn register_function_like_macro(&mut self, name: String, arguments: Vec<Token>, tokens: Vec<Token>)
    {
        self.defines.insert(name, MacroReplacements::FunctionLikeReplacement(arguments, tokens));
    }
}

