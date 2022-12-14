use {
    crate::registers::registers::Register,
    crate::instructions::instructions::InstructionName,
    super::utils::*,
};


#[derive(Debug)]
pub struct BType {
    pub name: InstructionName,
    pub reg_1: Register,
    pub reg_2: Register,
    pub label: usize,
}

impl BType {
    pub fn exec(&mut self) -> bool {
        match &self.name {
            InstructionName::Blt => exec_blt(self),
            InstructionName::Beq => exec_beq(self),
            InstructionName::Bgt => exec_bgt(self),
            _ => { return false; }
        }
    }
}
