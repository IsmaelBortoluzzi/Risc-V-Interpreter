use {
    crate::registers::registers::Register,
    crate::instructions::instructions::InstructionName,
    super::utils::*,
};



#[derive(Debug)]
pub struct SType {
    pub name: InstructionName,
    pub reg_1: Register,
    pub reg_2: Register,
    pub imm: i32,
}

impl SType {
    pub fn exec(&mut self) {
        match &self.name {
            InstructionName::Sw => exec_sw(self),
            InstructionName::Sb => exec_sb(self),
            _ => {}
        }
    }
}
