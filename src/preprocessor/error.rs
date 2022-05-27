use crate::{tokenizer::{Location, FileManager, Token}, errors::CompilerError};

/// Preprocessor Error
pub struct PreprocessorError
{
    pub error: PreprocessorErrorType,
    pub location: Location,
    pub arrow_length: usize
}

pub enum PreprocessorErrorType
{
    SyntaxError(String)
}

impl std::convert::Into<CompilerError> for PreprocessorError
{
    fn into(self) -> CompilerError
    {
        CompilerError::PreprocessorError(self)
    }
}

impl std::fmt::Display for PreprocessorError
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        match &self.error
        {
            PreprocessorErrorType::SyntaxError(text) => write!(f, "Syntax Error {}", text)?,
        }

        write!(f, " at {}", self.location)
    }
}

impl PreprocessorError
{
    pub fn syntax_error(error: String, token: &Token) -> Self
    {
        Self
        {
            location: token.location.clone(),
            error: PreprocessorErrorType::SyntaxError(error),
            arrow_length: token.code_styled().len()
        }
    }
}