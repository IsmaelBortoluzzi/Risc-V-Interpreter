use {
    crate::{
        instructions::{
            S_type::S_type::SType,
        },
        registers::registers::Register, 
    },
};


pub fn is_stack_operation(instr: &SType) -> bool {
    
    if instr.reg_2.name.as_str() != "sp" {
        return false;
    }
    true
}