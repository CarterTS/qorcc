use std::collections::HashMap;
use std::hash::Hash;

use crate::compiler::Compiler;
use crate::preprocessor::PreprocessorError;
use crate::tokenizer::Token;
use crate::tokenizer::FileManager;
use crate::errors::*;
use crate::tokenizer::TokenType;

/// Macro Replacements
#[derive(Debug, Clone)]
pub enum MacroReplacements
{
    None,
    DirectReplacement(Vec<Token>)
}

/// Preprocessor Context
pub struct PreprocessorContext<'a, 'b>
{
    compiler: &'a mut Compiler<'b>,
    defines: HashMap<String, MacroReplacements>
}

impl<'a, 'b> PreprocessorContext<'a, 'b>
{
    pub fn with_compiler(compiler: &'a mut Compiler<'b>) -> Self
    {
        Self
        {
            compiler,
            defines: HashMap::new()
        }
    }

    pub fn register_macro_empty(&mut self, name: String)
    {
        self.defines.insert(name, MacroReplacements::None);
    }

    pub fn register_direct_replace_macro(&mut self, name: String, tokens: Vec<Token>)
    {
        self.defines.insert(name, MacroReplacements::DirectReplacement(tokens));
    }
}

/// Preprocess a sequence of tokens into another sequence of tokens, this takes a compiler object as context as it must be able to reference other already tokenized files
pub fn preprocess(compiler: &mut Compiler, file: &FileManager) -> Result<Vec<Token>, PreprocessorError>
{
    let mut context = PreprocessorContext::with_compiler(compiler);

    let mut result = Vec::new();

    let mut peekable_iter = file.iter().peekable();

    while let Some(peeked_next) = peekable_iter.peek()
    {
        let current_line = peeked_next.location.line;

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
                        context.register_macro_empty(identifier);
                    }
                    // Otherwise, we will register the sequence of tokens as the macro
                    else
                    {
                        context.register_direct_replace_macro(identifier, macro_tokens);
                    }
                },
                _ => {return Err(PreprocessorError::syntax_error(format!("Error"),  &peeked_next))}
            }
            
            continue;
        }
        else if let TokenType::Identifier(identifier) = &peeked_next.token_type
        {
            if let Some(expand_to) = context.defines.get(identifier)
            {
                peekable_iter.next();

                match expand_to
                {
                    MacroReplacements::DirectReplacement(tokens) => result.append(&mut tokens.clone()),
                    _ => {}
                }
                continue;
            }
        }

        result.push(peekable_iter.next().unwrap().clone());
    }

    Ok(result)
}