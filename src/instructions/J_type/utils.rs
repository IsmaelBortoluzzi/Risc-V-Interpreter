use super::J_type::JType;


pub fn exec_jalr(instr: &mut JType) -> i64 {

    if instr.reg_1.name.as_str() != "zero" {
        instr.reg_1.value = instr.next_line.to_string();
    }
    
    instr.reg_2.value.parse::<i64>().expect("Not an integer!") + instr.offset
    
}


