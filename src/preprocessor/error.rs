use crate::{tokenizer::{Location, FileManager, Token, TokenType}, errors::CompilerError};

/// Preprocessor Error
pub struct PreprocessorError
{
    pub error: PreprocessorErrorType,
    pub location: Location,
    pub original_location: Option<Location>,
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

        write!(f, " at {}", self.location)?;

        if let Some(original) = &self.original_location
        {
            write!(f, " in macro expansion at {}", original)?;
        }

        Ok(())
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
            original_location: token.original_location.clone(),
            arrow_length: token.code_styled().len(),
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

    pub fn expect_string_literal(token: Option<&Token>) -> Result<Token, PreprocessorError>
    {
        let token = PreprocessorError::prevent_eof(token)?;

        if let TokenType::StringLiteral(_) = &token.token_type
        {
            Ok(token)
        }
        else
        {
            Err(PreprocessorError::syntax_error(format!("Expected string literal, got {}", token.code_styled()), &token))
        }
    }

    pub fn expect_symbol(token: Option<&Token>, symbol: &str) -> Result<(), PreprocessorError>
    {
        let token = PreprocessorError::prevent_eof(token)?;

        if let TokenType::Symbol(token_symbol) = &token.token_type
        {
            if token_symbol == symbol
            {
                return Ok(());
            }
        }

        Err(PreprocessorError::syntax_error(format!("Expected {}, got {}", symbol, token.code_styled()), &token))
    }
}