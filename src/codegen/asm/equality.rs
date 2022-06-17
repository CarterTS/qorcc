#![allow(dead_code)]
use crate::codegen::*;
use super::*;

impl AssemblyCodeGenerator
{
    fn add_equality_reg_reg(&self, destination: Register, src1: Register, src2: Register) -> String
    {
        format!("    xor {}, {}, {}\n    sltiu {}, {}, 1\n", destination, src1, src2, destination, destination)
    }

    fn add_equality_reg_imm(&self, destination: Register, src1: Register, src2: i64) -> String
    {
        format!("    xori {}, {}, {}\n    sltiu {}, {}, 1\n", destination, src1, src2, destination, destination)
    }

    pub fn add_equality(&self, destination: Register, src1: &IRValue, src2: &IRValue) -> String
    {
        match (src1, src2)
        {
            (IRValue::Register(src1), IRValue::Register(src2)) => self.add_equality_reg_reg(destination, *self.mapping.get(src1).unwrap(), *self.mapping.get(src2).unwrap()),
            (IRValue::Register(src1), IRValue::Immediate(src2)) => self.add_equality_reg_imm(destination, *self.mapping.get(src1).unwrap(), src2.value as i64),
            (IRValue::Immediate(src2), IRValue::Register(src1)) => self.add_equality_reg_imm(destination, *self.mapping.get(src1).unwrap(), src2.value as i64),
            (IRValue::Immediate(_), IRValue::Immediate(_)) => todo!(),
        }
    }
}