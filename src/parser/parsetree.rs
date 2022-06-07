#![allow(dead_code)]

use crate::tokenizer::Token;

use super::*;

/// Parsetree Node Types
#[derive(Debug, Clone)]
pub enum ParseTreeNode
{
    CompilationUnit{children: Vec<ParseTreeNode>},
    Function{name: String, return_type: ValueType, arguments: Vec<(String, ValueType, Token)>, child: Box<ParseTreeNode>, name_token: Token },
    StatementBlock{children: Vec<ParseTreeNode>},
    ReturnStatement{child: Option<Box<ParseTreeNode>>},
    ConstantExpression{value: Value, token: Token},
    VariableExpression{name: String, token: Token},
    PostfixExpression{operation: PostfixExpressionOperation, children: Vec<ParseTreeNode>, optoken: Token},
    UnaryExpression{operation: UnaryExpressionOperation, child: Box<ParseTreeNode>, optoken: Token},
    CastExpression{children: Vec<ParseTreeNode>},
    MultiplicativeExpression{operation: MultiplicativeExpressionOperation, children: Vec<ParseTreeNode>, optoken: Token},
    AdditiveExpression{operation: AdditiveExpressionOperation, children: Vec<ParseTreeNode>, optoken: Token},
    ShiftExpression{operation: ShiftExpressionOperation, children: Vec<ParseTreeNode>, optoken: Token},
    RelationalExpression{operation: RelationalExpressionOperation, children: Vec<ParseTreeNode>, optoken: Token},
    EqualityExpression{operation: EqualityExpressionOperation, children: Vec<ParseTreeNode>, optoken: Token},
    AndExpression{children: Vec<ParseTreeNode>, optoken: Token},
    XorExpression{children: Vec<ParseTreeNode>, optoken: Token},
    OrExpression{children: Vec<ParseTreeNode>, optoken: Token},
    LogicalAndExpression{children: Vec<ParseTreeNode>, optoken: Token},
    LogicalOrExpression{children: Vec<ParseTreeNode>, optoken: Token},
    ConditionalExpression{children: Vec<ParseTreeNode>, optoken: Token},
    AssignmentExpression{operation: AssignmentExpressionOperation, children: Vec<ParseTreeNode>, optoken: Token},
    CommaExpression{children: Vec<ParseTreeNode>, optoken: Token},
    IfStatement{children: Vec<ParseTreeNode>},
    WhileLoop{children: Vec<ParseTreeNode>},
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
            ParseTreeNode::ConstantExpression{ .. } => None,
            ParseTreeNode::VariableExpression { .. } => None,
            ParseTreeNode::PostfixExpression { children, .. } => Some(children.to_vec()),
            ParseTreeNode::UnaryExpression { child, .. } => Some(vec![(**child).clone()]),
            ParseTreeNode::CastExpression { children } => Some(children.to_vec()),
            ParseTreeNode::MultiplicativeExpression { children, .. } => Some(children.to_vec()),
            ParseTreeNode::AdditiveExpression { children, .. } => Some(children.to_vec()),
            ParseTreeNode::ShiftExpression { children, .. } => Some(children.to_vec()),
            ParseTreeNode::RelationalExpression { children, .. } => Some(children.to_vec()),
            ParseTreeNode::EqualityExpression { children, .. } => Some(children.to_vec()),
            ParseTreeNode::AndExpression { children, .. } => Some(children.to_vec()),
            ParseTreeNode::XorExpression { children, .. } => Some(children.to_vec()),
            ParseTreeNode::OrExpression { children, .. } => Some(children.to_vec()),
            ParseTreeNode::LogicalAndExpression { children, .. } => Some(children.to_vec()),
            ParseTreeNode::LogicalOrExpression { children, .. } => Some(children.to_vec()),
            ParseTreeNode::ConditionalExpression { children, .. } => Some(children.to_vec()),
            ParseTreeNode::AssignmentExpression { children, .. } => Some(children.to_vec()),
            ParseTreeNode::CommaExpression { children, .. } => Some(children.to_vec()),
            ParseTreeNode::IfStatement { children } => Some(children.to_vec()),
            ParseTreeNode::WhileLoop { children } => Some(children.to_vec()),
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
            ParseTreeNode::ConstantExpression{ value, .. } => write!(f, "Value {}", value),
            ParseTreeNode::VariableExpression{ name, .. } => write!(f, "Variable {}", name),
            ParseTreeNode::PostfixExpression { operation, .. } => write!(f, "PoastfixExpression {:?}", operation),
            ParseTreeNode::UnaryExpression { operation, .. } => write!(f, "UnaryExpression {:?}", operation),
            ParseTreeNode::CastExpression { .. } => write!(f, "CastExpression"),
            ParseTreeNode::MultiplicativeExpression { operation, .. } =>write!(f, "MultiplicativeExpression {:?}", operation),
            ParseTreeNode::AdditiveExpression { operation, .. } => write!(f, "AdditiveExpression {:?}", operation),
            ParseTreeNode::ShiftExpression { operation, .. } => write!(f, "ShiftExpression {:?}", operation),
            ParseTreeNode::RelationalExpression { operation, .. } => write!(f, "RelationalExpression {:?}", operation),
            ParseTreeNode::EqualityExpression { operation, .. } => write!(f, "EqualityExpression {:?}", operation),
            ParseTreeNode::AndExpression { .. } => write!(f, "AndExpression"),
            ParseTreeNode::XorExpression { .. } => write!(f, "XorExpression"),
            ParseTreeNode::OrExpression { .. } => write!(f, "OrExpression"),
            ParseTreeNode::LogicalAndExpression { .. } => write!(f, "LogicalAndExpression"),
            ParseTreeNode::LogicalOrExpression { .. } => write!(f, "LogicalOrExpression"),
            ParseTreeNode::ConditionalExpression { .. } => write!(f, "ConditionalExpression"),
            ParseTreeNode::AssignmentExpression { operation, .. } => write!(f, "AssignmentExpression {:?}", operation),
            ParseTreeNode::CommaExpression { .. } => write!(f, "CommaExpression"),
            ParseTreeNode::IfStatement { .. } => write!(f, "IfStatement"),
            ParseTreeNode::WhileLoop { .. } => write!(f, "WhileLoop"),
        }
    }
}

fn render_arguments(arguments: &Vec<(String, ValueType, Token)>) -> String
{
    let mut result = String::new();

    for (arg_name, arg_type, _) in arguments
    {
        if result.len() > 0
        {
            result += ", ";
        }

        result += &format!("{} {}", arg_name, arg_type);
    }

    result
}