use crate::tokenizer::Token;
use crate::errors::CompilerResult;

use super::*;

/// Parser Context Object
pub struct Parser
{
    stream: Vec<Token>
}

impl Parser
{
    pub fn from_stream(stream: Vec<Token>) -> Self
    {
        Self
        {
            stream
        }
    }

    /// Parse the roken stream
    pub fn parse(&mut self) -> CompilerResult<ParseTreeNode>
    {
        trace!("Starting Parsing");

        let constant = ParseTreeNode::ConstantExpression(Value::code_constant(42));

        Ok(constant)
    }
}