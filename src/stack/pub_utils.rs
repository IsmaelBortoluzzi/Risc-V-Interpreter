use {
    super::{
        stack::Stack,
        utils::*,
    },
    crate::{
        instructions::{
            instructions::InstructionName,
            S_type::S_type::SType,
        },
        registers::registers::Register, 
    },
    std::collections::HashMap,
};


pub fn is_stack_operation(instr: &SType) -> bool {
    
    if instr.reg_2.name.as_str() != "sp" {
        return false;
    }
    true
}