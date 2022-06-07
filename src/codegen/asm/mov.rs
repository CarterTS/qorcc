#![allow(dead_code)]
use crate::codegen::*;
use super::*;


impl AssemblyCodeGenerator
{
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
            IRValue::Register(reg) => self.move_reg_reg(dest, *self.mapping.get(&reg).unwrap()),
            IRValue::Immediate(immediate) => self.move_reg_imm(dest, immediate.value as i64),
        }
    }
}