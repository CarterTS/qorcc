#![allow(dead_code)]

use super::*;

/// Parsetree Node Types
#[derive(Debug, Clone)]
pub enum ParseTreeNode
{
    CompilationUnit{children: Vec<ParseTreeNode>},
    Function{name: String, return_type: ValueType, children: Vec<ParseTreeNode>},
    StatementBlock{children: Vec<ParseTreeNode>},
    ReturnStatement{children: Vec<ParseTreeNode>},
    ConstantExpression(Value)
}

impl ParseTreeNode
{
    /// Display a parse tree node
    pub fn display(&self)
    {
        self.internal_display("", true);
    }

    /// Internal display implementation
    fn internal_display(&self, indentation: &str, last: bool)
    {
        println!("{}{}╴{}", indentation, if last {"└"} else {"├"}, self);

        let next_indent = format!("{}│ ", indentation);
        let final_indent = format!("{}  ", indentation);

        if let Some(children) = self.get_children()
        {
            let length = children.len();
            for (i, child) in children.iter().enumerate()
            {
                child.internal_display(if last {&final_indent} else {&next_indent} , i == length - 1);
            }
        }
    }

    /// Get a reference to the children if available
    pub fn get_children(&self) -> Option<&Vec<ParseTreeNode>>
    {
        match self
        {
            ParseTreeNode::CompilationUnit { children } => Some(&children),
            ParseTreeNode::Function { children, .. } => Some(&children),
            ParseTreeNode::StatementBlock { children } => Some(&children),
            ParseTreeNode::ReturnStatement { children } => Some(&children),
            ParseTreeNode::ConstantExpression(_) => None,
        }
    }
}

impl std::fmt::Display for ParseTreeNode
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        match &self
        {
            ParseTreeNode::CompilationUnit { .. } => write!(f, "CompilationUnit"),
            ParseTreeNode::Function { name, return_type, .. } => write!(f, "Function {} -> {}", name, return_type),
            ParseTreeNode::StatementBlock { .. } => write!(f, "StatementBlock"),
            ParseTreeNode::ReturnStatement { .. } => write!(f, "ReturnStatement"),
            ParseTreeNode::ConstantExpression(value) => write!(f, "Value {}", value),
        }
    }
}