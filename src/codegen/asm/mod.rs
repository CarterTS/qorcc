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

    pub fn add_reg_imm(&self, dest: Register, src1: Register, src2: i64) -> String
    {
        format!("    addi {}, {}, {}\n", dest, src1, src2)
    }

    pub fn add_reg_reg(&self, dest: Register, src1: Register, src2: Register) -> String
    {
        format!("    add {}, {}, {}\n", dest, src1, src2)
    }

    pub fn sub_reg_imm(&self, dest: Register, src1: Register, src2: i64) -> String
    {
        format!("    addi {}, {}, {}\n", dest, src1, -src2)
    }

    pub fn sub_imm_reg(&self, dest: Register, src1: i64, src2: Register) -> String
    {
        format!("    neg t6, {}\n    addi {}, {}, t6\n", src2, dest, src1)
    }

    pub fn sub_reg_reg(&self, dest: Register, src1: Register, src2: Register) -> String
    {
        format!("    sub {}, {}, {}\n", dest, src1, src2)
    }

    pub fn mul_reg_imm(&self, dest: Register, src1: Register, src2: i64) -> String
    {
        format!("    li t6, {}\n    mul {}, {}, t6\n", src2, dest, src1)
    }

    pub fn mul_reg_reg(&self, dest: Register, src1: Register, src2: Register) -> String
    {
        format!("    mul {}, {}, {}\n", dest, src1, src2)
    }

    pub fn div_reg_imm(&self, dest: Register, src1: Register, src2: i64) -> String
    {
        format!("    li t6, {}\n    div {}, {}, t6\n", src2, dest, src1)
    }

    pub fn div_imm_reg(&self, dest: Register, src1: i64, src2: Register) -> String
    {
        format!("    li t6, {}\n    div {}, {}, t6\n", src1, dest, src2)
    }

    pub fn div_reg_reg(&self, dest: Register, src1: Register, src2: Register) -> String
    {
        format!("    div {}, {}, {}\n", dest, src1, src2)
    }

    pub fn mod_reg_imm(&self, dest: Register, src1: Register, src2: i64) -> String
    {
        format!("    li t6, {}\n    rem {}, {}, t6\n", src2, dest, src1)
    }

    pub fn mod_imm_reg(&self, dest: Register, src1: i64, src2: Register) -> String
    {
        format!("    li t6, {}\n    div {}, t6, {}\n", src1, dest, src2)
    }

    pub fn mod_reg_reg(&self, dest: Register, src1: Register, src2: Register) -> String
    {
        format!("    rem {}, {}, {}\n", dest, src1, src2)
    }

    pub fn move_reg_value(&self, dest: Register, source: IRValue) -> String
    {
        match source
        {
            super::IRValue::Register(reg) => self.move_reg_reg(dest, *self.mapping.get(&reg).unwrap()),
            super::IRValue::Immediate(immediate) => self.move_reg_imm(dest, immediate.value as i64),
        }
    }

    pub fn add_reg_value_value(&self, dest: Register, src1: &IRValue, src2: &IRValue) -> String
    {
        if let (IRValue::Immediate(imm0), IRValue::Immediate(imm1)) = (src1, src2)
        {
            self.move_reg_imm(dest, imm0.value.wrapping_add(imm1.value) as i64)
        }
        else if let (IRValue::Immediate(imm), IRValue::Register(reg)) = (src1, src2)
        {
            self.add_reg_imm(dest, *self.mapping.get(&reg).unwrap(), imm.value as i64)
        }
        else if let (IRValue::Register(reg), IRValue::Immediate(imm)) = (src1, src2)
        {
            self.add_reg_imm(dest, *self.mapping.get(&reg).unwrap(), imm.value as i64)
        }
        else if let (IRValue::Register(reg0), IRValue::Register(reg1)) = (src1, src2)
        {
            self.add_reg_reg(dest, *self.mapping.get(&reg0).unwrap(), *self.mapping.get(&reg1).unwrap())
        }
        else
        {
            unreachable!()
        }
    }

    pub fn mul_reg_value_value(&self, dest: Register, src1: &IRValue, src2: &IRValue) -> String
    {
        if let (IRValue::Immediate(imm0), IRValue::Immediate(imm1)) = (src1, src2)
        {
            self.move_reg_imm(dest, imm0.value.wrapping_mul(imm1.value) as i64)
        }
        else if let (IRValue::Immediate(imm), IRValue::Register(reg)) = (src1, src2)
        {
            self.mul_reg_imm(dest, *self.mapping.get(&reg).unwrap(), imm.value as i64)
        }
        else if let (IRValue::Register(reg), IRValue::Immediate(imm)) = (src1, src2)
        {
            self.mul_reg_imm(dest, *self.mapping.get(&reg).unwrap(), imm.value as i64)
        }
        else if let (IRValue::Register(reg0), IRValue::Register(reg1)) = (src1, src2)
        {
            self.mul_reg_reg(dest, *self.mapping.get(&reg0).unwrap(), *self.mapping.get(&reg1).unwrap())
        }
        else
        {
            unreachable!()
        }
    }

    pub fn sub_reg_value_value(&self, dest: Register, src1: &IRValue, src2: &IRValue) -> String
    {
        if let (IRValue::Immediate(imm0), IRValue::Immediate(imm1)) = (src1, src2)
        {
            self.move_reg_imm(dest, imm0.value.wrapping_sub(imm1.value) as i64)
        }
        else if let (IRValue::Immediate(imm), IRValue::Register(reg)) = (src1, src2)
        {
            self.sub_reg_imm(dest, *self.mapping.get(&reg).unwrap(), imm.value as i64)
        }
        else if let (IRValue::Register(reg), IRValue::Immediate(imm)) = (src1, src2)
        {
            self.sub_imm_reg(dest, imm.value as i64, *self.mapping.get(&reg).unwrap())
        }
        else if let (IRValue::Register(reg0), IRValue::Register(reg1)) = (src1, src2)
        {
            self.sub_reg_reg(dest, *self.mapping.get(&reg0).unwrap(), *self.mapping.get(&reg1).unwrap())
        }
        else
        {
            unreachable!()
        }
    }

    pub fn div_reg_value_value(&self, dest: Register, src1: &IRValue, src2: &IRValue) -> String
    {
        if let (IRValue::Immediate(imm0), IRValue::Immediate(imm1)) = (src1, src2)
        {
            self.move_reg_imm(dest, imm0.value.wrapping_sub(imm1.value) as i64)
        }
        else if let (IRValue::Immediate(imm), IRValue::Register(reg)) = (src1, src2)
        {
            self.div_reg_imm(dest, *self.mapping.get(&reg).unwrap(), imm.value as i64)
        }
        else if let (IRValue::Register(reg), IRValue::Immediate(imm)) = (src1, src2)
        {
            self.div_imm_reg(dest, imm.value as i64, *self.mapping.get(&reg).unwrap())
        }
        else if let (IRValue::Register(reg0), IRValue::Register(reg1)) = (src1, src2)
        {
            self.div_reg_reg(dest, *self.mapping.get(&reg0).unwrap(), *self.mapping.get(&reg1).unwrap())
        }
        else
        {
            unreachable!()
        }
    }

    pub fn mod_reg_value_value(&self, dest: Register, src1: &IRValue, src2: &IRValue) -> String
    {
        if let (IRValue::Immediate(imm0), IRValue::Immediate(imm1)) = (src1, src2)
        {
            self.move_reg_imm(dest, imm0.value.wrapping_sub(imm1.value) as i64)
        }
        else if let (IRValue::Immediate(imm), IRValue::Register(reg)) = (src1, src2)
        {
            self.mod_reg_imm(dest, *self.mapping.get(&reg).unwrap(), imm.value as i64)
        }
        else if let (IRValue::Register(reg), IRValue::Immediate(imm)) = (src1, src2)
        {
            self.mod_imm_reg(dest, imm.value as i64, *self.mapping.get(&reg).unwrap())
        }
        else if let (IRValue::Register(reg0), IRValue::Register(reg1)) = (src1, src2)
        {
            self.mod_reg_reg(dest, *self.mapping.get(&reg0).unwrap(), *self.mapping.get(&reg1).unwrap())
        }
        else
        {
            unreachable!()
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
            IRInstruction::Add { dest, src1, src2  } => 
            {
                if let IRValue::Register(dest) = dest
                {
                    Ok(self.add_reg_value_value(*self.mapping.get(dest).unwrap(), src1, src2))
                }
                else
                {
                    unreachable!()
                }
            },
            IRInstruction::Sub { dest, src1, src2  } => 
            {
                if let IRValue::Register(dest) = dest
                {
                    Ok(self.sub_reg_value_value(*self.mapping.get(dest).unwrap(), src1, src2))
                }
                else
                {
                    unreachable!()
                }
            },
            IRInstruction::Mul { dest, src1, src2  } => 
            {
                if let IRValue::Register(dest) = dest
                {
                    Ok(self.mul_reg_value_value(*self.mapping.get(dest).unwrap(), src1, src2))
                }
                else
                {
                    unreachable!()
                }
            },
            IRInstruction::Div { dest, src1, src2  } => 
            {
                if let IRValue::Register(dest) = dest
                {
                    Ok(self.div_reg_value_value(*self.mapping.get(dest).unwrap(), src1, src2))
                }
                else
                {
                    unreachable!()
                }
            },
            IRInstruction::Mod { dest, src1, src2  } => 
            {
                if let IRValue::Register(dest) = dest
                {
                    Ok(self.mod_reg_value_value(*self.mapping.get(dest).unwrap(), src1, src2))
                }
                else
                {
                    unreachable!()
                }
            },
            _ => todo!()
        }
    }
}