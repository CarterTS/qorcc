#![allow(dead_code)]

/// Expression Operations for postfix Expressions
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PostfixExpressionOperation
{
    ArrayIndexing,
    FunctionCall,
    MemberAccess,
    IndirectMemberAccess,
    Increment,
    Decrement,
    InitializerList
}

/// Expression Operations for Unary Expressions
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnaryExpressionOperation
{
    Increment,
    Decrement,
    Dereference,
    Reference,
    Positive,
    Negation,
    BitwiseNot,
    LogicalNot
}

/// Expression Operations for Multiplicative Expressions
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MultiplicativeExpressionOperation
{
    Multiplication,
    Division,
    Modulus
}

/// Expression Operations for Additive Expressions
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdditiveExpressionOperation
{
    Addition,
    Subtraction
}

/// Expression Operations for Shift Expressions
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShiftExpressionOperation
{
    ShiftLeft,
    ShiftRight
}

/// Expression Operations for Relational Expressions
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RelationalExpressionOperation
{
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual
}

/// Expression Operations for Equality Expressions
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EqualityExpressionOperation
{
    Equality,
    Nonequality
}

/// Expression Operations for Assignment Expressions
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssignmentExpressionOperation
{
    Assignment,
    MultiplicationAssignment,
    DivisionAssignment,
    ModulusAssignment,
    AdditionAssignment,
    SubtractionAssignment,
    ShiftLeftAssignment,
    ShiftRightAssignment,
    AndAssignment,
    XorAssignment,
    OrAssignment
}