use std::collections::HashMap;

use crate::dot_data::data::DotDataVariable;

use {
    crate::registers::registers::Register,
    crate::instructions::instructions::InstructionName,
    super::utils::*,
};



#[derive(Debug)]
pub struct SType<'a> {
    pub name: InstructionName,
    pub reg_1: &'a mut Register,
    pub reg_2: Register,
    pub imm: i32,
}

impl SType<'_> {
    pub fn exec(&mut self, data: &mut HashMap<String, DotDataVariable>) {
        match &self.name {
            InstructionName::Sw => exec_sw(self, data),
            InstructionName::Sb => exec_sb(self, data),
            _ => {}
        }
    }
}
