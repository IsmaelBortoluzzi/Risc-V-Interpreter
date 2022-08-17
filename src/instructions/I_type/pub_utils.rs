use std::collections::HashMap;

use crate::{instructions::instructions::{InstructionName, get_read_reg}, registers::registers::Register};

use super::I_type::IType;

pub fn _exec_i_type(instruction: &Vec<&str>, registers: &mut HashMap<String, Register>) {
    let instr_reg_1: Vec<&str> = instruction[0].split(" ").collect();
    let instr: InstructionName = InstructionName::str_to_instr_name(instr_reg_1[0]);
    
    let imm: i32 = instruction[2].trim().parse().unwrap();
    let reg_2: Register = get_read_reg(instruction[1].trim(),registers);
    let reg_1: &mut Register = registers.get_mut(instr_reg_1[1].trim()).expect("Unknown Register");

    let mut r_type_instr: IType = IType {
        name: instr,
        reg_1: reg_1.clone(),
        reg_2: reg_2,
        imm: imm,
    };

    r_type_instr.exec();
    reg_1.copy_attrs(&r_type_instr.reg_1);

}