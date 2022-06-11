#![allow(dead_code)]
use crate::codegen::*;
use super::*;

impl AssemblyCodeGenerator
{
    pub fn add_jump(&self, branch_index: usize, function: &IRFunction) -> String
    {
        format!("    j {}\n", self.block_label(branch_index, function))
    }
}