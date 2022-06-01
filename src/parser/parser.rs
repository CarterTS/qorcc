use crate::tokenizer::{Token, TokenType};
use crate::errors::CompilerResult;

use super::*;

/// Parser Context Object
pub struct Parser<'a, S: std::iter::Iterator<Item = &'a Token>>
{
    stream: std::iter::Peekable<S>
}

impl<'a, S: std::iter::Iterator<Item = &'a Token>> Parser<'a, S>
{
    pub fn from_stream(stream: S) -> Self
    {
        Self
        {
            stream: stream.peekable()
        }
    }

    /// Parse the roken stream
    pub fn parse(&mut self) -> CompilerResult<ParseTreeNode>
    {
        trace!("Starting Parsing");

        /*
        let constant = ParseTreeNode::ConstantExpression(Value::code_constant(42));
        let return_statement = ParseTreeNode::ReturnStatement { children: vec![constant] };
        let block_statement = ParseTreeNode::StatementBlock { children: vec![return_statement] };
        let function = ParseTreeNode::Function { name: String::from("main"), children: vec![block_statement] };
        let translation = ParseTreeNode::CompilationUnit { children: vec![function] };

        println!("{}", self.parse_type()?);

        Ok(translation) */

        self.parse_compilation_unit()
    }

    /// Parse a compilation unit
    pub fn parse_compilation_unit(&mut self) -> CompilerResult<ParseTreeNode>
    {
        let mut children = Vec::new();

        while let Some(token) = self.stream.peek()
        {
            if token.is_eof()
            {
                break;
            }

            children.push(self.parse_function_definition()?);
        }

        Ok(ParseTreeNode::CompilationUnit { children: children })
    }

    /// Parse a function definition
    pub fn parse_function_definition(&mut self) -> CompilerResult<ParseTreeNode>
    {
        let return_type = self.parse_type()?;

        let name = ParseError::expect_named_identifier(self.stream.next(), "function name")?.code_styled();

        ParseError::expect_symbol(self.stream.next(), "(")?;
        ParseError::expect_symbol(self.stream.next(), ")")?;
        
        let statement = self.parse_statement()?;

        Ok(ParseTreeNode::Function { name: name, child: Box::new(statement), return_type })
    }

    /// Parse a statement
    pub fn parse_statement(&mut self) -> CompilerResult<ParseTreeNode>
    {
        let peeked = ParseError::prevent_eof(self.stream.peek().map(|v| *v))?;

        // Compound statement
        if peeked.token_type == TokenType::Symbol(String::from("{"))
        {
            let mut children = Vec::new();

            ParseError::expect_symbol(self.stream.next(), "{")?;

            loop
            {
                let peeked = ParseError::prevent_eof(self.stream.peek().map(|v| *v))?;

                if peeked.token_type == TokenType::Symbol(String::from("}"))
                {
                    break;
                }

                children.push(self.parse_statement()?);
            }

            ParseError::expect_symbol(self.stream.next(), "}")?;

            Ok(ParseTreeNode::StatementBlock { children })
        }
        // Return statement
        else if peeked.token_type == TokenType::Identifier(String::from("return"))
        {
            self.stream.next();

            let value = self.parse_value()?;

            ParseError::expect_symbol(self.stream.next(), ";")?;

            Ok(ParseTreeNode::ReturnStatement { child: Some(Box::new(value)) })
        }
        else
        {
            Err(ParseError::syntax_error(format!("Expected statement, got {}", peeked.code_styled()), &peeked).into())
        }
    }

    /// Parse a value
    pub fn parse_value(&mut self) -> CompilerResult<ParseTreeNode>
    {
        let number = ParseError::expect_integer_literal(self.stream.next())?;

        if let TokenType::IntegerLiteral(number) = number.token_type
        {
            Ok(ParseTreeNode::ConstantExpression(Value::code_constant(number as u32)))
        }
        else
        {
            unreachable!()
        }
    }

    /// Parse a type from the stream
    pub fn parse_type(&mut self) -> CompilerResult<ValueType>
    {
        // Types must start with an identifier, struct, enum, or the name of the type
        let token = ParseError::expect_named_identifier(self.stream.next(), "type")?;

        let raw_type = match token.code_styled().as_str()
        {
            "struct" =>
            {
                let struct_name = ParseError::expect_identifier(self.stream.next())?;

                RawValueType::Struct(struct_name.code_styled())
            },
            "void" => RawValueType::Void,
            "char" => RawValueType::I8,
            "short" => RawValueType::I16,
            "int" => RawValueType::I32,
            "long" => RawValueType::I64,
            _ => 
            {
                return Err(ParseError::syntax_error(format!("Expected type, got {}", token.code_styled()), &token).into())
            }
        };
        
        let mut reference_count = 0;

        while let Some(token) = self.stream.peek()
        {
            if token.token_type == TokenType::Symbol(format!("*"))
            {
                self.stream.next();
                reference_count += 1;
            }
            else
            {
                break;
            }
        }

        Ok(ValueType{references: reference_count, value_type: raw_type})
    }
}