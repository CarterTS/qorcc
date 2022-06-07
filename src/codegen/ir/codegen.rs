use crate::parser::*;
use crate::tokenizer::*;
use crate::errors::*;

use super::*;

/// Convert a parse tree into intermediate representation
pub fn parse_tree_to_ir(tree: ParseTreeNode) -> CompilerResult<IR>
{
    trace!("Convert to Intermediate Representation");

    let mut ir = IR::new();

    if let ParseTreeNode::CompilationUnit { children } = tree
    {
        for child in children
        {
            ir.functions.push(parse_tree_function_to_ir(child)?);
        }
    }
    else
    {
        panic!("Expected a CompilationUnit parse tree node, got {}", tree);
    }

    Ok(ir)
}

/// Convert a function parse tree node into an IRFunction
pub fn parse_tree_function_to_ir(tree: ParseTreeNode) -> CompilerResult<IRFunction>
{
    if let ParseTreeNode::Function { name, return_type, child, arguments, .. } = tree
    {
        IRFunction::with_statement_and_args(name, return_type, *child, arguments)
    }
    else
    {
        panic!()
    }
}

impl IRFunction
{
    pub fn with_statement_and_args(name: String, return_type: ValueType, statement: ParseTreeNode, arguments: Vec<(String, ValueType, Token)>) -> CompilerResult<Self>
    {
        let mut result = Self
        {
            name,
            return_type,
            blocks: vec![IRBlock::new(0)],
            current_block: 0,
            scope_stack: Vec::new(),
            next_register: 0
        };

        let scope = IRScope::from_arguments(arguments, &mut result);

        result.scope_stack.push(scope);

        result.add_statement(statement)?;

        Ok(result)
    }

    pub fn get_variable_value(&mut self, expression: ParseTreeNode) -> CompilerResult<IRValue>
    {
        if let ParseTreeNode::VariableExpression { name, token } = expression
        {
            println!("Trying to dereference token for {}", token);
            
            let mut result = None;

            for scope in self.scope_stack.iter().rev()
            {
                result = result.or(scope.access_variable(&name));
                if result.is_some()
                {
                    break;
                }
            }

            if let Some(result_value) = result
            {
                Ok(result_value)
            }
            else
            {
                Err(CodegenError::compile_error(format!("Variable {} is not defined", name), &token).into())
            }
        }
        else
        {
            panic!()
        }
    }

    pub fn generate_expression(&mut self, expression: ParseTreeNode) -> CompilerResult<IRValue>
    {
        match expression
        {
            ParseTreeNode::ConstantExpression{ value, .. } => Ok(IRValue::Immediate(value)),
            ParseTreeNode::VariableExpression { .. } => self.get_variable_value(expression),
            _ => 
            {
                error!("Unhandled Expression Type {}", expression);
                todo!()
            }
        }
    }

    pub fn add_statement(&mut self, statement: ParseTreeNode) -> CompilerResult<()>
    {
        match statement
        {
            ParseTreeNode::StatementBlock { children } => 
            {
                for child in children
                {
                    self.add_statement(child)?;
                }

                Ok(())
            },
            ParseTreeNode::ReturnStatement { child } => 
            {
                if let Some(expression) = child
                {
                    let value = self.generate_expression(*expression)?;
                    self.mut_current_block().add_instruction(IRInstruction::Return { value });
                }
                else
                {
                    self.mut_current_block().add_instruction(IRInstruction::Return { value: IRValue::Immediate(Value::code_constant(0)) });
                }

                Ok(())
            },
            _ => panic!(),
        }
    }
}