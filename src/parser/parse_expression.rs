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

    /// Parse a postfix expression
    pub fn parse_postfix_expression(&mut self) -> CompilerResult<ParseTreeNode>
    {
        self.parse_primary_expression()
    }

    /// Parse an unary expression
    pub fn parse_unary_expression(&mut self) -> CompilerResult<ParseTreeNode>
    {
        let peeked_next = ParseError::prevent_eof(self.stream.peek().map(|v| *v))?;

        // Check for the operation
        let operation = match &peeked_next.token_type
        {
            TokenType::Symbol(symbol_data) => 
            {
                match symbol_data.as_str()
                {
                    "-" => UnaryExpressionOperation::Negation,
                    "+" => UnaryExpressionOperation::Positive,
                    "--" => UnaryExpressionOperation::Decrement,
                    "++" => UnaryExpressionOperation::Increment,
                    "&" => UnaryExpressionOperation::Reference,
                    "*" => UnaryExpressionOperation::Dereference,
                    "~" => UnaryExpressionOperation::BitwiseNot,
                    "!" => UnaryExpressionOperation::LogicalNot,
                    _ => { return self.parse_postfix_expression(); }
                }
            }
            _ => { return self.parse_postfix_expression(); },
        };

        // Step through the operation token
        let optoken = ParseError::prevent_eof(self.stream.next())?;

        // Get the inner operation
        let inner = self.parse_unary_expression()?;

        Ok(ParseTreeNode::UnaryExpression { operation, child: Box::new(inner), optoken })
    }

    /// Parse a cast expression
    pub fn parse_cast_expression(&mut self) -> CompilerResult<ParseTreeNode>
    {
        self.parse_unary_expression()
    }

    /// Parse a multiplicative expression
    pub fn parse_multiplicative_expression(&mut self) -> CompilerResult<ParseTreeNode>
    {
        // Get the first part of the expression
        let mut first = self.parse_cast_expression()?;

        loop
        {
            // Peek the operation
            let peeked_next = ParseError::prevent_eof(self.stream.peek().map(|v| *v))?;

            // Get the operation
            let operation = match peeked_next.token_type
            {
                TokenType::Symbol(symbol_text) =>
                match symbol_text.as_str()
                {
                    "*" => MultiplicativeExpressionOperation::Multiplication,
                    "/" => MultiplicativeExpressionOperation::Division,
                    "%" => MultiplicativeExpressionOperation::Modulus,
                    _ => { return Ok( first ) }
                }
                _ => { return Ok( first ) }
            };

            // Get the operation token
            let optoken = ParseError::prevent_eof(self.stream.next())?;

            // Get the second part of the expression
            let second = self.parse_cast_expression()?;

            first = ParseTreeNode::MultiplicativeExpression { operation, children: vec![first, second], optoken };
        }
    }

    /// Parse an additive expression
    pub fn parse_additive_expression(&mut self) -> CompilerResult<ParseTreeNode>
    {
        // Get the first part of the expression
        let mut first = self.parse_multiplicative_expression()?;

        loop
        {
            // Peek the operation
            let peeked_next = ParseError::prevent_eof(self.stream.peek().map(|v| *v))?;

            // Get the operation
            let operation = match peeked_next.token_type
            {
                TokenType::Symbol(symbol_text) =>
                match symbol_text.as_str()
                {
                    "+" => AdditiveExpressionOperation::Addition,
                    "-" => AdditiveExpressionOperation::Subtraction,
                    _ => { return Ok( first ) }
                }
                _ => { return Ok( first ) }
            };

            // Get the operation token
            let optoken = ParseError::prevent_eof(self.stream.next())?;

            // Get the second part of the expression
            let second = self.parse_multiplicative_expression()?;

            first = ParseTreeNode::AdditiveExpression { operation, children: vec![first, second], optoken };
        }
    }

    /// Parse a shift expression
    pub fn parse_shift_expression(&mut self) -> CompilerResult<ParseTreeNode>
    {
        self.parse_additive_expression()
    }   

    /// Parse a relational expression
    pub fn parse_relational_expression(&mut self) -> CompilerResult<ParseTreeNode>
    {
        self.parse_shift_expression()
    }

    /// Parse an equality expression
    pub fn parse_equality_expression(&mut self) -> CompilerResult<ParseTreeNode>
    {
        self.parse_relational_expression()
    }

    /// Parse an and expression
    pub fn parse_and_expression(&mut self) -> CompilerResult<ParseTreeNode>
    {
        self.parse_equality_expression()
    }

    /// Parse a xor expression
    pub fn parse_xor_expression(&mut self) -> CompilerResult<ParseTreeNode>
    {
        self.parse_and_expression()
    }

    /// Parse an or expression
    pub fn parse_or_expression(&mut self) -> CompilerResult<ParseTreeNode>
    {
        self.parse_xor_expression()
    }

    /// Parse a logical and expression
    pub fn parse_logical_and_expression(&mut self) -> CompilerResult<ParseTreeNode>
    {
        self.parse_or_expression()
    }

    /// Parse a logical or expression
    pub fn parse_logical_or_expression(&mut self) -> CompilerResult<ParseTreeNode>
    {
        self.parse_logical_and_expression()
    }

    /// Parse a conditional expression
    pub fn parse_conditional_expression(&mut self) -> CompilerResult<ParseTreeNode>
    {
        self.parse_logical_or_expression()
    }

    /// Parse an assignment expression
    pub fn parse_assignment_expression(&mut self) -> CompilerResult<ParseTreeNode>
    {
        self.parse_conditional_expression()
    }

    /// Parse a comma expression
    pub fn parse_comma_expression(&mut self) -> CompilerResult<ParseTreeNode>
    {
        self.parse_assignment_expression()
    }
}