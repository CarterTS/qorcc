use crate::tokenizer::{Token, TokenType};
use crate::errors::CompilerResult;

use super::*;

impl<'a, S: std::iter::Iterator<Item = &'a Token>> Parser<'a, S>
{
    /// Parse a primary expression
    pub fn parse_primary_expression(&mut self) -> CompilerResult<ParseTreeNode>
    {
        let peeked_next = ParseError::prevent_eof(self.stream.peek().map(|v| *v))?;

        if peeked_next.token_type == TokenType::Symbol("(".to_string())
        {
            self.stream.next();

            let result = self.parse_expression()?;

            ParseError::expect_symbol(self.stream.next(), ")")?;

            return Ok(result)
        }

        match &peeked_next.token_type
        {
            TokenType::Identifier(_) => self.parse_identifier("variable name"),
            TokenType::IntegerLiteral(_) => self.parse_integer_value(),
            _ => Err(ParseError::syntax_error(format!("Expected primary expression, got {}", peeked_next.code_styled()), &peeked_next).into())
        }
    }

    /// Parse a postfix expression
    pub fn parse_postfix_expression(&mut self) -> CompilerResult<ParseTreeNode>
    {
        // Get the first part of the expression
        let mut first = self.parse_primary_expression()?;

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
                    "[" => PostfixExpressionOperation::ArrayIndexing,
                    "(" => PostfixExpressionOperation::FunctionCall,
                    "." => PostfixExpressionOperation::MemberAccess,
                    "->" => PostfixExpressionOperation::IndirectMemberAccess,
                    "++" => PostfixExpressionOperation::Increment,
                    "--" => PostfixExpressionOperation::Decrement,
                    _ => { return Ok( first ) }
                }
                _ => { return Ok( first ) }
            };

            // Get the operation token
            let optoken = ParseError::prevent_eof(self.stream.next())?;

            // Construct the children portion
            let mut children = vec![first];

            // Parse the proper value for each expression type
            match operation
            {
                PostfixExpressionOperation::ArrayIndexing => 
                {
                    children.push(self.parse_expression()?);
                    ParseError::expect_symbol(self.stream.next(), "]")?;
                },
                PostfixExpressionOperation::FunctionCall => 
                {
                    while self.stream.peek().map(|v| &v.token_type) != Some(&TokenType::Symbol(")".to_string()))
                    {
                        children.push(self.parse_expression()?);

                        if self.stream.peek().map(|v| &v.token_type) != Some(&TokenType::Symbol(",".to_string()))
                        {
                            break;
                        }

                        ParseError::expect_symbol(self.stream.next(), ",")?;
                    }

                    ParseError::expect_symbol(self.stream.next(), ")")?;
                },
                PostfixExpressionOperation::MemberAccess => 
                {
                    children.push(self.parse_identifier("member name")?);
                },
                PostfixExpressionOperation::IndirectMemberAccess => 
                {
                    children.push(self.parse_identifier("member name")?);
                },
                PostfixExpressionOperation::InitializerList => todo!(),
                _ => {},
            }

            first = ParseTreeNode::PostfixExpression { operation, children, optoken };
        }
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
        let inner = self.parse_cast_expression()?;

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
        // Get the first part of the expression
        let mut first = self.parse_additive_expression()?;

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
                    "<<" => ShiftExpressionOperation::ShiftLeft,
                    ">>" => ShiftExpressionOperation::ShiftRight,
                    _ => { return Ok( first ) }
                }
                _ => { return Ok( first ) }
            };

            // Get the operation token
            let optoken = ParseError::prevent_eof(self.stream.next())?;

            // Get the second part of the expression
            let second = self.parse_additive_expression()?;

            first = ParseTreeNode::ShiftExpression { operation, children: vec![first, second], optoken };
        }
    }   

    /// Parse a relational expression
    pub fn parse_relational_expression(&mut self) -> CompilerResult<ParseTreeNode>
    {
        // Get the first part of the expression
        let mut first = self.parse_shift_expression()?;

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
                    "<" => RelationalExpressionOperation::LessThan,
                    "<=" => RelationalExpressionOperation::LessThanOrEqual,
                    ">" => RelationalExpressionOperation::GreaterThan,
                    ">=" => RelationalExpressionOperation::GreaterThanOrEqual,
                    _ => { return Ok( first ) }
                }
                _ => { return Ok( first ) }
            };

            // Get the operation token
            let optoken = ParseError::prevent_eof(self.stream.next())?;

            // Get the second part of the expression
            let second = self.parse_shift_expression()?;

            first = ParseTreeNode::RelationalExpression { operation, children: vec![first, second], optoken };
        }
    }

    /// Parse an equality expression
    pub fn parse_equality_expression(&mut self) -> CompilerResult<ParseTreeNode>
    {
        // Get the first part of the expression
        let mut first = self.parse_relational_expression()?;

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
                    "==" => EqualityExpressionOperation::Equality,
                    "!=" => EqualityExpressionOperation::Nonequality,
                    _ => { return Ok( first ) }
                }
                _ => { return Ok( first ) }
            };

            // Get the operation token
            let optoken = ParseError::prevent_eof(self.stream.next())?;

            // Get the second part of the expression
            let second = self.parse_relational_expression()?;

            first = ParseTreeNode::EqualityExpression { operation, children: vec![first, second], optoken };
        }
    }

    /// Parse an and expression
    pub fn parse_and_expression(&mut self) -> CompilerResult<ParseTreeNode>
    {
        // Get the first part of the expression
        let mut first = self.parse_equality_expression()?;

        loop
        {
            // Peek the operation
            let peeked_next = ParseError::prevent_eof(self.stream.peek().map(|v| *v))?;

            // Get the operation
            if peeked_next.token_type != TokenType::Symbol("&".to_string())
            {
                return Ok(first);
            };

            // Get the operation token
            let optoken = ParseError::prevent_eof(self.stream.next())?;

            // Get the second part of the expression
            let second = self.parse_equality_expression()?;

            first = ParseTreeNode::AndExpression { children: vec![first, second], optoken };
        }
    }

    /// Parse a xor expression
    pub fn parse_xor_expression(&mut self) -> CompilerResult<ParseTreeNode>
    {
        // Get the first part of the expression
        let mut first = self.parse_and_expression()?;

        loop
        {
            // Peek the operation
            let peeked_next = ParseError::prevent_eof(self.stream.peek().map(|v| *v))?;

            // Get the operation
            if peeked_next.token_type != TokenType::Symbol("^".to_string())
            {
                return Ok(first);
            };

            // Get the operation token
            let optoken = ParseError::prevent_eof(self.stream.next())?;

            // Get the second part of the expression
            let second = self.parse_and_expression()?;

            first = ParseTreeNode::XorExpression { children: vec![first, second], optoken };
        }
    }

    /// Parse an or expression
    pub fn parse_or_expression(&mut self) -> CompilerResult<ParseTreeNode>
    {
        // Get the first part of the expression
        let mut first = self.parse_xor_expression()?;

        loop
        {
            // Peek the operation
            let peeked_next = ParseError::prevent_eof(self.stream.peek().map(|v| *v))?;

            // Get the operation
            if peeked_next.token_type != TokenType::Symbol("|".to_string())
            {
                return Ok(first);
            };

            // Get the operation token
            let optoken = ParseError::prevent_eof(self.stream.next())?;

            // Get the second part of the expression
            let second = self.parse_xor_expression()?;

            first = ParseTreeNode::OrExpression { children: vec![first, second], optoken };
        }
    }

    /// Parse a logical and expression
    pub fn parse_logical_and_expression(&mut self) -> CompilerResult<ParseTreeNode>
    {
        // Get the first part of the expression
        let mut first = self.parse_or_expression()?;

        loop
        {
            // Peek the operation
            let peeked_next = ParseError::prevent_eof(self.stream.peek().map(|v| *v))?;

            // Get the operation
            if peeked_next.token_type != TokenType::Symbol("&&".to_string())
            {
                return Ok(first);
            };

            // Get the operation token
            let optoken = ParseError::prevent_eof(self.stream.next())?;

            // Get the second part of the expression
            let second = self.parse_or_expression()?;

            first = ParseTreeNode::LogicalAndExpression { children: vec![first, second], optoken };
        }
    }

    /// Parse a logical or expression
    pub fn parse_logical_or_expression(&mut self) -> CompilerResult<ParseTreeNode>
    {
        // Get the first part of the expression
        let mut first = self.parse_logical_and_expression()?;

        loop
        {
            // Peek the operation
            let peeked_next = ParseError::prevent_eof(self.stream.peek().map(|v| *v))?;

            // Get the operation
            if peeked_next.token_type != TokenType::Symbol("||".to_string())
            {
                return Ok(first);
            };

            // Get the operation token
            let optoken = ParseError::prevent_eof(self.stream.next())?;

            // Get the second part of the expression
            let second = self.parse_logical_and_expression()?;

            first = ParseTreeNode::LogicalOrExpression { children: vec![first, second], optoken };
        }
    }

    /// Parse a conditional expression
    pub fn parse_conditional_expression(&mut self) -> CompilerResult<ParseTreeNode>
    {
        // Get the first part of the expression
        let first = self.parse_logical_or_expression()?;

        // Peek the operation
        let peeked_next = ParseError::prevent_eof(self.stream.peek().map(|v| *v))?;

        // Get the operation
        if peeked_next.token_type != TokenType::Symbol("?".to_string())
        {
            return Ok(first);
        };

        // Get the operation token
        let optoken = ParseError::prevent_eof(self.stream.next())?;

        // Get the second part of the expression
        let second = self.parse_expression()?;

        ParseError::expect_symbol(self.stream.next(), ":")?;

        let third = self.parse_expression()?;

        Ok(ParseTreeNode::ConditionalExpression { children: vec![first, second, third], optoken })
    }

    /// Parse an assignment expression
    pub fn parse_assignment_expression(&mut self) -> CompilerResult<ParseTreeNode>
    {
        // Get the first part of the expression
        let mut first = self.parse_conditional_expression()?;

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
                    "=" => AssignmentExpressionOperation::Assignment,
                    "*=" => AssignmentExpressionOperation::MultiplicationAssignment,
                    "/=" => AssignmentExpressionOperation::DivisionAssignment,
                    "%=" => AssignmentExpressionOperation::ModulusAssignment,
                    "+=" => AssignmentExpressionOperation::AdditionAssignment,
                    "-=" => AssignmentExpressionOperation::SubtractionAssignment,
                    "<<=" => AssignmentExpressionOperation::ShiftLeftAssignment,
                    ">>=" => AssignmentExpressionOperation::ShiftRightAssignment,
                    "&=" => AssignmentExpressionOperation::AndAssignment,
                    "^=" => AssignmentExpressionOperation::XorAssignment,
                    "|=" => AssignmentExpressionOperation::OrAssignment,
                    _ => { return Ok( first ) }
                }
                _ => { return Ok( first ) }
            };

            // Get the operation token
            let optoken = ParseError::prevent_eof(self.stream.next())?;

            // Get the second part of the expression
            let second = self.parse_conditional_expression()?;

            first = ParseTreeNode::AssignmentExpression { operation, children: vec![first, second], optoken };
        }
    }

    /// Parse a comma expression
    pub fn parse_comma_expression(&mut self) -> CompilerResult<ParseTreeNode>
    {
        // Get the first part of the expression
        let mut first = self.parse_assignment_expression()?;

        loop
        {
            // Peek the operation
            let peeked_next = ParseError::prevent_eof(self.stream.peek().map(|v| *v))?;

            // Get the operation
            if peeked_next.token_type != TokenType::Symbol(",".to_string())
            {
                return Ok(first);
            };

            // Get the operation token
            let optoken = ParseError::prevent_eof(self.stream.next())?;

            // Get the second part of the expression
            let second = self.parse_assignment_expression()?;

            first = ParseTreeNode::CommaExpression { children: vec![first, second], optoken };
        }
    }
}