#![allow(dead_code)]
use super::{IR, IRBlock, IRFunction, IRInstruction};

use crate::errors::*;

/// Assembly Code Generator for RISC-V
#[derive(Debug, Clone)]
pub struct AssemblyCodeGenerator
{
    ir: IR,
}

impl AssemblyCodeGenerator
{
    pub fn from_ir(ir: IR) -> Self
    {
        Self
        {
            ir
        }
    }

    pub fn codegen(&self) -> CompilerResult<String>
    {
        trace!("Generating Assembly");

        let mut result = String::new();

        for function in &self.ir.functions
        {
            result += &self.emit_function(function)?;
            result += "\n\n";
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

    pub fn emit_instruction(&self, inst: &IRInstruction, _block: &IRBlock, _function: &IRFunction) -> CompilerResult<String>
    {
        match inst
        {
            IRInstruction::Return { value } => 
            {
                match value
                {
                    super::IRValue::Register(_) => todo!(),
                    super::IRValue::Immediate(immediate) => Ok(format!("    li a0, {}\n    ret\n", immediate.value)),
                }
            },
        }
    }
}