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
}

/// Preprocess a sequence of tokens into another sequence of tokens, this takes a compiler object as context as it must be able to reference other already tokenized files
pub fn preprocess(compiler: &mut Compiler, file: &FileManager) -> CompilerResult<Vec<Token>>
{
    let mut context = PreprocessorContext::with_compiler(compiler);

    let mut result = Vec::new();

    let mut peekable_iter = file.iter().peekable();

    while let Some(peeked_next) = peekable_iter.peek()
    {
        if let TokenType::PreprocessorDirective(directive) = &peeked_next.token_type
        {
            println!("Got preprocessor directive {}", directive);
            return Err(PreprocessorError::syntax_error(format!("Error"),  &peeked_next).into())
        }

        result.push(peekable_iter.next().unwrap().clone())
    }

    Ok(result)
}