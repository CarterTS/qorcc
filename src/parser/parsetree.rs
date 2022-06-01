#![allow(dead_code)]

use super::*;

/// Parsetree Node Types
#[derive(Debug, Clone)]
pub enum ParseTreeNode
{
    CompilationUnit{children: Vec<ParseTreeNode>},
    Function{name: String, return_type: ValueType, arguments: Vec<(String, ValueType)>, child: Box<ParseTreeNode>},
    StatementBlock{children: Vec<ParseTreeNode>},
    ReturnStatement{child: Option<Box<ParseTreeNode>>},
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
    pub fn get_children(&self) -> Option<Vec<ParseTreeNode>>
    {
        match self
        {
            ParseTreeNode::CompilationUnit { children } => Some(children.to_vec()),
            ParseTreeNode::Function { child, .. } => Some(vec![(**child).clone()]),
            ParseTreeNode::StatementBlock { children } => Some(children.to_vec()),
            ParseTreeNode::ReturnStatement { child } => child.as_ref().map(|c| vec![(**c).clone()]),
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
            ParseTreeNode::Function { name, return_type, arguments, .. } => write!(f, "Function {}({}) -> {}", name, render_arguments(arguments), return_type),
            ParseTreeNode::StatementBlock { .. } => write!(f, "StatementBlock"),
            ParseTreeNode::ReturnStatement { .. } => write!(f, "ReturnStatement"),
            ParseTreeNode::ConstantExpression(value) => write!(f, "Value {}", value),
        }
    }
}

fn render_arguments(arguments: &Vec<(String, ValueType)>) -> String
{
    let mut result = String::new();

    for (arg_name, arg_type) in arguments
    {
        if result.len() > 0
        {
            result += ", ";
        }

        result += &format!("{} {}", arg_name, arg_type);
    }

    result
}