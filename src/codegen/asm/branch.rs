#![allow(dead_code)]
use crate::codegen::*;
use super::*;

use crate::parser::Value;

impl AssemblyCodeGenerator
{
    fn bne_reg_reg(&self, reg0: Register, reg1: Register, branch: usize, function: &IRFunction) -> String
    {
        format!("    bne {}, {}, {}\n", reg0, reg1, self.block_label(branch, function))
    }

    pub fn add_branch(&self, condition: IRBranchCondition, value0: IRValue, value1: IRValue, dest_true: usize, dest_false: usize, function: &IRFunction) -> String
    {
        if condition == IRBranchCondition::NotEqual && value1 == IRValue::Immediate(Value::code_constant(0))
        {
            if let IRValue::Register(reg0) = value0
            {
                self.bne_reg_reg(*self.mapping.get(&reg0).unwrap(), Register::Zero, dest_true, function) + &self.add_jump(dest_false, function)
            }
            else
            {
                todo!()
            }
        }
        else
        {
            todo!()
        }
    }
}