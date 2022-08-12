use {
    crate::registers::registers::Register,
    crate::instructions::instructions::InstructionName,
    super::utils::{ 

    },
};



#[derive(Debug)]
pub struct IType {
    pub name: InstructionName,
    pub reg_1: Register,
    pub reg_2: Register,
    pub imm: i32,
}

impl RType {
    pub fn exec(&mut self) {
        match &self.name {
            InstructionName::Add => exec_addi(self),
            InstructionName::Sub => exec_andi(self),
            InstructionName::Mul => exec_xori(self),
            InstructionName::Div => exec_ori(self),
            InstructionName::Rem => exec_slli(self),
            _ => {}
        }
    }
}
