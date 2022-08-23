use {
    std::collections::HashMap,
    crate::{
        registers::registers::Register,
        dot_data::data::DotDataVariable,
        instructions::instructions::InstructionName,
        stack::stack::Stack,
    },
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
    pub fn exec(&mut self, data: &mut HashMap<String, DotDataVariable>, stack: &mut Stack) {
        match &self.name {
            InstructionName::Sw => exec_sw(self, data, stack),
            InstructionName::Sb => exec_sb(self, data, stack),
            _ => {}
        }
    }
}
