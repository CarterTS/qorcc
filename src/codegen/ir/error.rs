#![allow(dead_code)]

use crate::tokenizer::*;

/// Parse Error
pub struct CodegenError
{
    pub error: CodegenErrorType,
    pub location: Location,
    pub original_location: Option<Location>,
    pub arrow_length: usize
}

pub enum CodegenErrorType
{
    CompileError(String)
}

impl std::fmt::Display for CodegenError
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        match &self.error
        {
            CodegenErrorType::CompileError(text) => write!(f, "{}", text)?,
        }

        write!(f, " at {}", self.location)?;

        if let Some(original) = &self.original_location
        {
            write!(f, " in macro expansion at {}", original)?;
        }

        Ok(())
    }
}

impl CodegenError
{
    pub fn compile_error(error: String, token: &Token) -> Self
    {
        Self
        {
            location: token.location.clone(),
            error: CodegenErrorType::CompileError(error),
            original_location: token.original_location.clone(),
            arrow_length: token.code_styled().len(),
        }
    }
}