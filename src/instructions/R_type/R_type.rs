use {
    crate::registers::registers::Register,
    crate::instructions::instructions::InstructionName,
    super::utils::{ 
        exec_add,
        exec_sub,
        exec_mul,
        exec_div,
        exec_rem,
        exec_sll,
        exec_xor,
        exec_or,
        exec_and,
    },
};



#[derive(Debug)]
pub struct RType {
    pub name: InstructionName,
    pub reg_1: Register,
    pub reg_2: Register,
    pub reg_3: Register,
}

impl RType {
    pub fn exec(&mut self) {
        match &self.name {
            InstructionName::Add => exec_add(self),
            InstructionName::Sub => exec_sub(self),
            InstructionName::Mul => exec_mul(self),
            InstructionName::Div => exec_div(self),
            InstructionName::Rem => exec_rem(self),
            InstructionName::Sll => exec_sll(self),
            InstructionName::Xor => exec_xor(self),
            InstructionName::Or => exec_or(self),
            InstructionName::And => exec_and(self),
            _ => {}
        }
    }
}
