use std::collections::HashMap;

use crate::tokenizer::*;
use crate::parser::*;

/// Intermediate Representation Structure
#[derive(Debug, Clone)]
pub struct IR
{
    pub functions: Vec<IRFunction>
}

impl IR
{
    pub fn new() -> Self
    {
        Self
        {
            functions: Vec::new()
        }
    }

    pub fn display(&self)
    {
        println!("Intermediate Representation:");

        for function in &self.functions
        {
            function.display();
        }
    }
}

/// Scope for the Intermediate Representation Code Generation
#[derive(Debug, Clone)]
pub struct IRScope
{
    variables: HashMap<String, usize>
}

impl IRScope
{
    #[allow(dead_code)]
    pub fn new() -> Self
    {
        Self
        {
            variables: HashMap::new()
        }
    }

    pub fn from_arguments(arguments: Vec<(String, ValueType, Token)>, function: &mut IRFunction) -> Self
    {
        let mut variables = HashMap::new();

        for (arg_name, _, _) in arguments
        {
            let reg = function.alloc_next_register();
            variables.insert(arg_name, reg);
        }

        Self
        {
            variables
        }
    }

    pub fn access_variable(&self, name: &str) -> Option<IRValue>
    {
        self.variables.get(name).map(|reg_num| IRValue::Register(*reg_num))
    }
}

/// Intermediate Representation Function
#[derive(Debug, Clone)]
pub struct IRFunction
{
    pub name: String,
    pub return_type: ValueType,
    pub blocks: Vec<IRBlock>,
    pub current_block: usize,
    pub scope_stack: Vec<IRScope>,
    pub next_register: usize,
    pub next_block: usize,
}

impl IRFunction
{
    pub fn display(&self)
    {
        println!("function {}() -> {}:", self.name, self.return_type);

        for block in &self.blocks
        {
            block.display();
        }
    }

    pub fn mut_current_block(&mut self) -> &mut IRBlock
    {
        &mut self.blocks[self.current_block]
    }

    pub fn alloc_next_register(&mut self) -> usize
    {
        self.next_register += 1;
        self.next_register - 1
    }

    pub fn alloc_next_block(&mut self) -> usize
    {
        self.next_block += 1;

        self.blocks.push(IRBlock::new(self.next_block - 1));

        self.next_block - 1
    }
}

/// Intermediate Representation Block 
#[derive(Debug, Clone)]
pub struct IRBlock
{
    pub label: String,
    pub instructions: Vec<IRInstruction>
}

impl IRBlock
{
    pub fn new(index: usize) -> Self
    {
        Self
        {
            label: format!("L{}", index),
            instructions: vec![],
        }
    }

    pub fn add_instruction(&mut self, instruction: IRInstruction)
    {
        self.instructions.push(instruction);
    }

    pub fn display(&self)
    {
        println!("  {}:", self.label);

        for (i, instruction) in self.instructions.iter().enumerate()
        {
            println!("    {:5} {}", format!("{}", i), instruction);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum IRValue
{
    Register(usize),
    Immediate(Value)
}

impl std::fmt::Display for IRValue
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        match self
        {
            IRValue::Register(number) => write!(f, "R{}", number),
            IRValue::Immediate(value) => write!(f, "{}", value),
        }
    } 
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IRBranchCondition
{
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanEqualTo,
    GreaterThanEqualTo
}

impl std::fmt::Display for IRBranchCondition
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        match self  
        {
            IRBranchCondition::Equal => write!(f, "eq"),
            IRBranchCondition::NotEqual => write!(f, "ne"),
            IRBranchCondition::LessThan => write!(f, "lt"),
            IRBranchCondition::GreaterThan => write!(f, "gt"),
            IRBranchCondition::LessThanEqualTo => write!(f, "le"),
            IRBranchCondition::GreaterThanEqualTo => write!(f, "ge"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum IRInstruction
{
    Return { value: IRValue },
    Add { dest: IRValue, src1: IRValue, src2: IRValue },
    Sub { dest: IRValue, src1: IRValue, src2: IRValue },
    Mul { dest: IRValue, src1: IRValue, src2: IRValue },
    Div { dest: IRValue, src1: IRValue, src2: IRValue },
    Mod { dest: IRValue, src1: IRValue, src2: IRValue },
    Jump { dest: usize },
    Branch { condition: IRBranchCondition, src1: IRValue,  src2: IRValue, dest_true: usize, dest_false: usize },
    Conditional { condition: IRBranchCondition, dest: IRValue, src1: IRValue, src2: IRValue },
    Backup { register: usize },
    Restore { register: usize },
    FunctionCall { name: String, arguments: Vec<IRValue> },
    LoadRet { dest: IRValue }
}

impl std::fmt::Display for IRInstruction
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        match self
        {
            IRInstruction::Return { value } => write!(f, "ret     {}", value),
            IRInstruction::Add { dest, src1, src2 } => write!(f, "add     {}, {}, {}", dest ,src1, src2),
            IRInstruction::Sub { dest, src1, src2 } => write!(f, "sub     {}, {}, {}", dest ,src1, src2),
            IRInstruction::Mul { dest, src1, src2 } => write!(f, "mul     {}, {}, {}", dest ,src1, src2),
            IRInstruction::Div { dest, src1, src2 } => write!(f, "div     {}, {}, {}", dest ,src1, src2),
            IRInstruction::Mod { dest, src1, src2 } => write!(f, "mod     {}, {}, {}", dest ,src1, src2),
            IRInstruction::Jump { dest } => write!(f, "j       L{}", dest),
            IRInstruction::Branch { condition, src1, src2, dest_true, dest_false } => write!(f, "b{}     {}, {}, L{}, L{}", condition, src1, src2, dest_true, dest_false),
            IRInstruction::Conditional { condition, dest, src1, src2 } => write!(f, "s{}     {}, {}, {}", condition, dest, src1, src2),
            IRInstruction::Backup { register } => write!(f, "backup  {}", IRValue::Register(*register)),
            IRInstruction::Restore { register } => write!(f, "restore {}", IRValue::Register(*register)),
            IRInstruction::LoadRet { dest } => write!(f, "loadret {}", dest),
            IRInstruction::FunctionCall { name, arguments } => 
            {
                write!(f, "call    {}(", name)?;

                for (i, arg) in arguments.iter().enumerate()
                {
                    if i > 0
                    {
                        write!(f, ", ")?;
                    }

                    write!(f, "{}", arg)?;
                }

                write!(f, ")")
            }
        }
    } 
}