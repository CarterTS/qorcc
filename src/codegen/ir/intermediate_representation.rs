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
            trace!("Inserting Variable {} into register {}", &arg_name, reg);
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

#[derive(Debug, Clone)]
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
            IRValue::Register(number) => write!(f, "{}", number),
            IRValue::Immediate(value) => write!(f, "{}", value),
        }
    } 
}

#[derive(Debug, Clone)]
pub enum IRInstruction
{
    Return { value: IRValue },
}

impl std::fmt::Display for IRInstruction
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        match self
        {
            IRInstruction::Return { value } => write!(f, "ret     {}", value),
        }
    } 
}