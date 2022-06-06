use crate::compiler::Compiler;
use crate::tokenizer::{Token, TokenType};
use crate::errors::CompilerResult;

use super::*;

impl<'a, S: std::iter::Iterator<Item = &'a Token>> Parser<'a, S>
{
    /// Parse a primary expression
    pub fn parse_primary_expression(&mut self) -> CompilerResult<ParseTreeNode>
    {
        let peeked_next = ParseError::prevent_eof(self.stream.peek().map(|v| *v))?;

        match &peeked_next.token_type
        {
            TokenType::Identifier(_) => self.parse_variable_name(),
            TokenType::IntegerLiteral(_) => self.parse_integer_value(),
            _ => Err(ParseError::syntax_error(format!("Expected primary expression, got {}", peeked_next.code_styled()), &peeked_next).into())
        }
    }
}