#![allow(dead_code)]

use crate::tokenizer::*;

/// Parse Error
pub struct ParseError
{
    pub error: ParseErrorType,
    pub location: Location,
    pub original_location: Option<Location>,
    pub arrow_length: usize
}

pub enum ParseErrorType
{
    SyntaxError(String)
}

impl std::fmt::Display for ParseError
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        match &self.error
        {
            ParseErrorType::SyntaxError(text) => write!(f, "Syntax Error {}", text)?,
        }

        write!(f, " at {}", self.location)?;

        if let Some(original) = &self.original_location
        {
            write!(f, " in macro expansion at {}", original)?;
        }

        Ok(())
    }
}

impl ParseError
{
    pub fn syntax_error(error: String, token: &Token) -> Self
    {
        Self
        {
            location: token.location.clone(),
            error: ParseErrorType::SyntaxError(error),
            original_location: token.original_location.clone(),
            arrow_length: token.code_styled().len(),
        }
    }

    pub fn prevent_eof(token: Option<&Token>) -> Result<Token, ParseError>
    {
        if let Some(token) = token
        {
            if token.token_type == TokenType::EndOfFile
            {
                return Err(ParseError::syntax_error("Unexpected end of file while parsing".to_string(), token));
            }

            Ok(token.clone())
        }
        else
        {
            unreachable!()
        }
    }

    pub fn expect_integer_literal(token: Option<&Token>) -> Result<Token, ParseError>
    {
        let token = ParseError::prevent_eof(token)?;

        if let TokenType::IntegerLiteral(_) = &token.token_type
        {
            Ok(token)
        }
        else
        {
            Err(ParseError::syntax_error(format!("Expected integer literal, got {}", token.code_styled()), &token))
        }
    }

    pub fn expect_identifier(token: Option<&Token>) -> Result<Token, ParseError>
    {
        let token = ParseError::prevent_eof(token)?;

        if let TokenType::Identifier(_) = &token.token_type
        {
            Ok(token)
        }
        else
        {
            Err(ParseError::syntax_error(format!("Expected identifier, got {}", token.code_styled()), &token))
        }
    }

    pub fn expect_specific_identifier(token: Option<&Token>, specific: &str) -> Result<Token, ParseError>
    {
        let token = ParseError::prevent_eof(token)?;

        if TokenType::Identifier(specific.to_string()) == token.token_type
        {
            Ok(token)
        }
        else
        {
            Err(ParseError::syntax_error(format!("Expected {}, got {}", specific, token.code_styled()), &token))
        }
    }

    pub fn expect_named_identifier(token: Option<&Token>, name: &str) -> Result<Token, ParseError>
    {
        let token = ParseError::prevent_eof(token)?;

        if let TokenType::Identifier(_) = &token.token_type
        {
            Ok(token)
        }
        else
        {
            Err(ParseError::syntax_error(format!("Expected {}, got {}", name, token.code_styled()), &token))
        }
    }

    pub fn expect_string_literal(token: Option<&Token>) -> Result<Token, ParseError>
    {
        let token = ParseError::prevent_eof(token)?;

        if let TokenType::StringLiteral(_) = &token.token_type
        {
            Ok(token)
        }
        else
        {
            Err(ParseError::syntax_error(format!("Expected string literal, got {}", token.code_styled()), &token))
        }
    }

    pub fn expect_symbol(token: Option<&Token>, symbol: &str) -> Result<(), ParseError>
    {
        let token = ParseError::prevent_eof(token)?;

        if let TokenType::Symbol(token_symbol) = &token.token_type
        {
            if token_symbol == symbol
            {
                return Ok(());
            }
        }

        Err(ParseError::syntax_error(format!("Expected {}, got {}", symbol, token.code_styled()), &token))
    }
}