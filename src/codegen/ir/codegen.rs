use crate::parser::*;
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
    if let ParseTreeNode::Function { name, return_type, child } = tree
    {
        Ok(IRFunction::with_statement(name, return_type, *child))
    }
    else
    {
        panic!()
    }
}

impl IRFunction
{
    pub fn with_statement(name: String, return_type: ValueType, statement: ParseTreeNode) -> Self
    {
        let mut result = Self
        {
            name,
            return_type,
            blocks: vec![IRBlock::new(0)],
            current_block: 0
        };

        result.add_statement(statement);

        result
    }

    pub fn add_statement(&mut self, statement: ParseTreeNode)
    {
        match statement
        {
            ParseTreeNode::StatementBlock { children } => 
            {
                for child in children
                {
                    self.add_statement(child);
                }
            },
            ParseTreeNode::ReturnStatement { child } => 
            {
                if let Some(ParseTreeNode::ConstantExpression(value)) = child.map(|v| *v)
                {
                    self.mut_current_block().add_instruction(IRInstruction::Return { value: IRValue::Immediate(value) });
                }
            },
            _ => panic!(),
        }
    }
}