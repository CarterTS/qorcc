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
    A6,
    A7,
    T0,
    T1,
    T2,
    T3,
    T4,
    T5
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
            Register::A7 => write!(f, "a7"),
            Register::T0 => write!(f, "t0"),
            Register::T1 => write!(f, "t1"),
            Register::T2 => write!(f, "t2"),
            Register::T3 => write!(f, "t3"),
            Register::T4 => write!(f, "t4"),
            Register::T5 => write!(f, "t5"),
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
        default_mapping.insert(7, Register::A7);
        default_mapping.insert(8, Register::T0);
        default_mapping.insert(9, Register::T1);
        default_mapping.insert(10, Register::T2);
        default_mapping.insert(11, Register::T3);
        default_mapping.insert(12, Register::T4);
        default_mapping.insert(13, Register::T5);

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
            _ => todo!()
        }
    }
}