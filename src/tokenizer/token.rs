#![allow(dead_code)]

use super::Location;

/// Token types and their respective associated data
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType
{
    EndOfFile,
    Identifier(String),
    IntegerLiteral(u64),
    PreprocessorDirective(String),
    StringLiteral(String),
    CharacterLiteral(String),
    Symbol(String),
}

impl std::fmt::Display for TokenType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        match self
        {
            TokenType::EndOfFile => write!(f, "EOF"),
            TokenType::Identifier(name) => write!(f, "Identifier({})", name),
            TokenType::IntegerLiteral(int) => write!(f, "IntegerLiteral({})", int),
            TokenType::StringLiteral(string) => write!(f, "StringLiteral({})", string),
            TokenType::CharacterLiteral(character) => write!(f, "CharacterLiteral({})", character),
            TokenType::PreprocessorDirective(directive) => write!(f, "PreprocessorDirective({})", directive),
            TokenType::Symbol(symbol) => write!(f, "Symbol({})", symbol),
        }
    }
}

impl TokenType
{
    pub fn code_styled(&self) -> String
    {
        match self
        {
            TokenType::EndOfFile => String::new(),
            TokenType::Identifier(name) => name.clone(),
            TokenType::IntegerLiteral(int) => int.to_string(),
            TokenType::StringLiteral(string) => format!("\"{}\"", string),
            TokenType::CharacterLiteral(character) => format!("'{}'", character),
            TokenType::PreprocessorDirective(directive) => directive.clone(),
            TokenType::Symbol(symbol) => symbol.clone(),
        }
    }
}

/// Token object
#[derive(Debug, Clone)]
pub struct Token
{
    pub token_type: TokenType,
    pub location: Location,
    pub original_location: Option<Location>
}

impl Token
{
    pub fn eof(location: Location) -> Self
    {
        Self
        {
            token_type: TokenType::EndOfFile,
            location,
            original_location: None,
        }
    }

    pub fn construct(token_type: TokenType, location: Location) -> Self
    {
        Self
        {
            token_type,
            location,
            original_location: None,
        }
    }

    pub fn is_eof(&self) -> bool
    {
        matches!(self.token_type, TokenType::EndOfFile)
    }

    pub fn is_identifier(&self) -> bool
    {
        matches!(self.token_type, TokenType::Identifier(_))
    }

    pub fn is_integer(&self) -> bool
    {
        matches!(self.token_type, TokenType::IntegerLiteral(_))
    }

    pub fn is_preprocessor_directive(&self) -> bool
    {
        matches!(self.token_type, TokenType::PreprocessorDirective(_))
    }

    pub fn get_original(&self) -> &Location
    {
        self.original_location.as_ref().unwrap_or(&self.location)
    }
}

impl std::fmt::Display for Token
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "{} at {}", self.token_type, self.location)
    }
}

impl Token
{
    pub fn code_styled(&self) -> String
    {
        self.token_type.code_styled()
    }
}