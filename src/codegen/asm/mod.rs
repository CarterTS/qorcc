#![allow(dead_code)]
use super::IR;

use crate::errors::*;

/// Assembly Code Generator for RISC-V
#[derive(Debug, Clone)]
pub struct AssemblyCodeGenerator
{
    ir: IR
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

        todo!()
    }
}