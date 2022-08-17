use {
    crate::registers::registers::Register,
    crate::instructions::instructions::InstructionName,
    super::utils::*,
};


#[derive(Debug)]
pub struct JType {
    pub name: InstructionName,
    pub reg_1: Register,
    pub reg_2: Register,
    pub label: usize,
    pub next_line: usize,
}

impl JType {
    pub fn exec(&mut self) {
        match &self.name {
            /* InstructionName::J => exec_j(self), */  //  J is too simple to need this
            /* InstructionName::Jal => exec_jal(self), */ // Jal is too
            InstructionName::Jalr => exec_jalr(self),
            _ => {  }
        }
    }
}
