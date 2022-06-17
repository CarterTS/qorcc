#![allow(dead_code)]
use crate::codegen::*;
use super::*;

impl AssemblyCodeGenerator
{
    fn add_less_than_reg_reg(&self, destination: Register, src1: Register, src2: Register) -> String
    {
        format!("    slt {}, {}, {}\n", destination, src1, src2)
    }

    fn add_less_than_reg_imm(&self, destination: Register, src1: Register, src2: i64) -> String
    {
        format!("    slti {}, {}, {}\n", destination, src1, src2)
    }

    fn add_less_than_or_equal_reg_reg(&self, destination: Register, src1: Register, src2: Register) -> String
    {
        self.add_less_than_reg_reg(destination, src2, src1) + &format!("    xori {}, {}, 1\n", destination, destination)
    }

    fn add_less_than_or_equal_reg_imm(&self, destination: Register, src1: Register, src2: i64) -> String
    {
        format!("    slti {}, {}, {}\n", destination, src1, src2 + 1)
    }

    fn add_greater_than_reg_reg(&self, destination: Register, src1: Register, src2: Register) -> String
    {
        self.add_less_than_reg_reg(destination, src2, src1)
    }

    fn add_greater_than_reg_imm(&self, destination: Register, src1: Register, src2: i64) -> String
    {
        self.add_less_than_reg_imm(destination, src1, src2 + 1) + &format!("    xori {}, {}, 1\n", destination, destination)
    }

    fn add_greater_than_or_equal_reg_reg(&self, destination: Register, src1: Register, src2: Register) -> String
    {
        self.add_less_than_reg_reg(destination, src1, src2) + &format!("    xori {}, {}, 1\n", destination, destination)
    }

    fn add_greater_than_or_equal_reg_imm(&self, destination: Register, src1: Register, src2: i64) -> String
    {
        self.add_less_than_reg_imm(destination, src1, src2) + &format!("    xori {}, {}, 1\n", destination, destination)
    }

    pub fn add_less_than(&self, destination: Register, src1: &IRValue, src2: &IRValue) -> String
    {
        match (src1, src2)
        {
            (IRValue::Register(src1), IRValue::Register(src2)) => self.add_less_than_reg_reg(destination, *self.mapping.get(src1).unwrap(), *self.mapping.get(src2).unwrap()),
            (IRValue::Register(src1), IRValue::Immediate(src2)) => self.add_less_than_reg_imm(destination, *self.mapping.get(src1).unwrap(), src2.value as i64),
            (IRValue::Immediate(src2), IRValue::Register(src1)) => self.add_greater_than_reg_imm(destination, *self.mapping.get(src1).unwrap(), src2.value as i64),
            (IRValue::Immediate(_), IRValue::Immediate(_)) => todo!(),
        }
    }

    pub fn add_greater_than_equal(&self, destination: Register, src1: &IRValue, src2: &IRValue) -> String
    {
        match (src1, src2)
        {
            (IRValue::Register(src1), IRValue::Register(src2)) => self.add_greater_than_or_equal_reg_reg(destination, *self.mapping.get(src1).unwrap(), *self.mapping.get(src2).unwrap()),
            (IRValue::Register(src1), IRValue::Immediate(src2)) => self.add_greater_than_or_equal_reg_imm(destination, *self.mapping.get(src1).unwrap(), src2.value as i64),
            (IRValue::Immediate(src2), IRValue::Register(src1)) => self.add_less_than_or_equal_reg_imm(destination, *self.mapping.get(src1).unwrap(), src2.value as i64),
            (IRValue::Immediate(_), IRValue::Immediate(_)) => todo!(),
        }
    }
}