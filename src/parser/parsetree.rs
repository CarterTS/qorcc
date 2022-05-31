#![allow(dead_code)]

use super::*;

/// Parsetree Node Types
#[derive(Debug, Clone)]
pub enum ParseTreeNode
{
    CompilationUnit{children: Vec<ParseTreeNode>},
    Function{name: String},
    StatementBlock{children: Vec<ParseTreeNode>},
    ReturnStatement{children: Vec<ParseTreeNode>},
    ConstantExpression(Value)
}

impl ParseTreeNode
{
    /// Display a parse tree node
    pub fn display(&self)
    {
        println!("{:?}", self);
    }
}