use super::Location;

/// Token types and their respective associated data
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType
{
    EndOfFile,
    Identifier(String),
    IntegerLiteral(u64),
    StringLiteral(String),
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
            TokenType::Symbol(symbol) => write!(f, "Symbol({})", symbol),
        }
    }
}


/// Token object
#[derive(Debug, Clone)]
pub struct Token
{
    token_type: TokenType,
    location: Location
}

impl Token
{
    pub fn eof(location: Location) -> Self
    {
        Self
        {
            token_type: TokenType::EndOfFile,
            location
        }
    }

    pub fn construct(token_type: TokenType, location: Location) -> Self
    {
        Self
        {
            token_type,
            location
        }
    }
}

impl std::fmt::Display for Token
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "{} at {}", self.token_type, self.location)
    }
}