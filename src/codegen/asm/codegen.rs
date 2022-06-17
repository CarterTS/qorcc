#![allow(dead_code)]
use crate::codegen::{IR, IRBlock, IRFunction, IRInstruction, IRValue};
use super::*;

use std::collections::HashMap;

use crate::errors::*;

/// Assembly Code Generator for RISC-V
#[derive(Debug, Clone)]
pub struct AssemblyCodeGenerator
{
    pub ir: IR,
    pub mapping: HashMap<usize, Register>
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

        result += &format!("  {}:\n", self.block_label_raw(block, function));

        for inst in &block.instructions
        {
            result += &self.emit_instruction(inst, block, function)?;
        }

        Ok(result)
    }

    pub fn block_label_raw(&self, block: &IRBlock, function: &IRFunction) -> String
    {
        format!("__{}_{}", function.name, block.label)
    }

    pub fn block_label(&self, block: usize, function: &IRFunction) -> String
    {
        self.block_label_raw(&function.blocks[block], function)
    }

    pub fn emit_instruction(&self, inst: &IRInstruction, _block: &IRBlock, function: &IRFunction) -> CompilerResult<String>
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
            IRInstruction::Jump { dest } =>
            {
                Ok(self.add_jump(*dest, function))
            },
            IRInstruction::Branch { condition, src1, src2, dest_true, dest_false } =>
            {
                Ok(self.add_branch(*condition, src1.clone(), src2.clone(), *dest_true, *dest_false, function))
            },
            IRInstruction::Conditional { condition, dest, src1, src2 } =>
            {
                if let IRValue::Register(dest) = dest
                {
                    Ok(match condition
                    {
                        crate::codegen::IRBranchCondition::Equal => self.add_equality(*self.mapping.get(dest).unwrap(), src1, src2),
                        crate::codegen::IRBranchCondition::NotEqual => self.add_inequality(*self.mapping.get(dest).unwrap(), src1, src2),
                        crate::codegen::IRBranchCondition::LessThan => self.add_less_than(*self.mapping.get(dest).unwrap(), src1, src2),
                        crate::codegen::IRBranchCondition::GreaterThan => self.add_less_than(*self.mapping.get(dest).unwrap(), src2, src1),
                        crate::codegen::IRBranchCondition::LessThanEqualTo => self.add_greater_than_equal(*self.mapping.get(dest).unwrap(), src2, src1),
                        crate::codegen::IRBranchCondition::GreaterThanEqualTo => self.add_greater_than_equal(*self.mapping.get(dest).unwrap(), src1, src2),
                    })
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