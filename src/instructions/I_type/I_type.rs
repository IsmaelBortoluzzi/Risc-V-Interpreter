use {
    crate::registers::registers::Register,
    crate::instructions::instructions::InstructionName,
    super::utils::*,
};



#[derive(Debug)]
pub struct IType {
    pub name: InstructionName,
    pub reg_1: Register,
    pub reg_2: Register,
    pub imm: i32,
}

impl IType {
    pub fn exec(&mut self) {
        match &self.name {
            InstructionName::Addi => exec_addi(self),
            InstructionName::Slli => exec_slli(self),
            InstructionName::Xori => exec_xori(self),
            InstructionName::Ori => exec_ori(self),
            InstructionName::Andi => exec_andi(self),
            _ => {}
        }
    }
}
