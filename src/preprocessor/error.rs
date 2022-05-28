use crate::{tokenizer::{Location, FileManager, Token, TokenType}, errors::CompilerError};

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

    pub fn prevent_eof(token: Option<&Token>) -> Result<Token, PreprocessorError>
    {
        if let Some(token) = token
        {
            if token.token_type == TokenType::EndOfFile
            {
                return Err(PreprocessorError::syntax_error("Unexpected end of file while preprocessing".to_string(), token));
            }

            Ok(token.clone())
        }
        else
        {
            unreachable!()
        }
    }

    pub fn expect_identifier(token: Option<&Token>) -> Result<Token, PreprocessorError>
    {
        let token = PreprocessorError::prevent_eof(token)?;

        if let TokenType::Identifier(_) = &token.token_type
        {
            Ok(token)
        }
        else
        {
            Err(PreprocessorError::syntax_error(format!("Expected identifier, got {}", token.code_styled()), &token))
        }
    }
}