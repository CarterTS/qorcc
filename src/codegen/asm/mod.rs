#![allow(dead_code)]
use super::{IR, IRBlock, IRFunction, IRInstruction, IRValue};

use std::collections::HashMap;

use crate::errors::*;

/// RISC-V Registers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Register
{
    A0,
    A1,
    A2, 
    A3,
    A4,
    A5,
    A6
}

impl std::fmt::Display for Register
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        match self
        {
            Register::A0 => write!(f, "a0"),
            Register::A1 => write!(f, "a1"),
            Register::A2 => write!(f, "a2"),
            Register::A3 => write!(f, "a3"),
            Register::A4 => write!(f, "a4"),
            Register::A5 => write!(f, "a5"),
            Register::A6 => write!(f, "a6"),
        }
    }
}

/// Assembly Code Generator for RISC-V
#[derive(Debug, Clone)]
pub struct AssemblyCodeGenerator
{
    ir: IR,
    mapping: HashMap<usize, Register>
}

impl AssemblyCodeGenerator
{
    pub fn from_ir(ir: IR) -> Self
    {
        let mut default_mapping = HashMap::new();

        default_mapping.insert(0, Register::A0);
        default_mapping.insert(1, Register::A1);
        default_mapping.insert(2, Register::A2);
        default_mapping.insert(3, Register::A3);
        default_mapping.insert(4, Register::A4);
        default_mapping.insert(5, Register::A5);
        default_mapping.insert(6, Register::A6);

        Self
        {
            ir,
            mapping: default_mapping
        }
    }

    pub fn codegen(&self) -> CompilerResult<String>
    {
        trace!("Generating Assembly");

        let mut result = String::new();

        for function in &self.ir.functions
        {
            if result.len() > 0
            {  
                result += "\n";
            }
            
            result += &self.emit_function(function)?;
        }

        Ok(result)
    }

    pub fn emit_function(&self, function: &IRFunction) -> CompilerResult<String>
    {
        let name = &function.name;

        let mut result = String::new();

        result += &format!(".globl {}\n{}:\n", name, name);

        for block in &function.blocks
        {
            result += &self.emit_block(block, function)?;
        }

        Ok(result)
    }

    pub fn emit_block(&self, block: &IRBlock, function: &IRFunction) -> CompilerResult<String>
    {
        let mut result = String::new();

        result += &format!("  __{}_{}:\n", function.name, block.label);

        for inst in &block.instructions
        {
            result += &self.emit_instruction(inst, block, function)?;
        }

        Ok(result)
    }

    pub fn move_reg_reg(&self, dest: Register, source: Register) -> String
    {
        if dest != source
        {
            format!("    mv {}, {}\n", dest, source)
        }
        else 
        {
            String::new()
        }
    }

    pub fn move_reg_imm(&self, dest: Register, source: i64) -> String
    {
        format!("    li {}, {}\n", dest, source)
    }

    pub fn move_reg_value(&self, dest: Register, source: IRValue) -> String
    {
        match source
        {
            super::IRValue::Register(reg) => self.move_reg_reg(dest, *self.mapping.get(&reg).unwrap()),
            super::IRValue::Immediate(immediate) => self.move_reg_imm(dest, immediate.value as i64),
        }
    }

    pub fn emit_instruction(&self, inst: &IRInstruction, _block: &IRBlock, _function: &IRFunction) -> CompilerResult<String>
    {
        match inst
        {
            IRInstruction::Return { value } => 
            {
                Ok(self.move_reg_value(Register::A0, value.clone()) + "    ret\n")
            },
        }
    }
}