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

    #[allow(dead_code)]
    pub fn example() -> Self
    {
        let value = IRValue::Immediate(Value::code_constant(128));
        let return_inst = IRInstruction::Return { value };
        let block = IRBlock { label: format!("L0"), instructions: vec![return_inst] };
        let function = IRFunction { name: format!("main"), return_type: RawValueType::I32.into(), blocks: vec![block], current_block: 0 };

        Self
        {
            functions: vec![function]
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

/// Intermediate Representation Function
#[derive(Debug, Clone)]
pub struct IRFunction
{
    pub name: String,
    pub return_type: ValueType,
    pub blocks: Vec<IRBlock>,
    pub current_block: usize
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