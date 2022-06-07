#![allow(dead_code)]
use crate::codegen::*;
use super::*;


impl AssemblyCodeGenerator
{
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
}