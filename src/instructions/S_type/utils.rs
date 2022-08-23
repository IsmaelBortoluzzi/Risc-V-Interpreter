use {
    std::collections::HashMap,
    super::S_type::SType,
    crate::{
        dot_data::data::{
            DotDataVariable, 
            Type,
        },
        stack::{
            stack::Stack,
            pub_utils::*,
        },
    },
};


fn exec_sw_stack(stack: &mut Stack, instr: &mut SType) { 
    let mut address = instr.reg_2.value.parse::<usize>().expect("sp value should be a number!");

    if (address - 983616) % 4 != 0 {
        panic!("Memory unaligned!");
    }
    
    if address < 983616 { // 983616 is the min address (16kb of stack is allocated)
        panic!("Stack overflow!");
    }
    else if address > 1000000 {
        panic!("Address does not belong to stack!");
    }
    
    address = (address - 983616 - 1/*1 less than len*/) / 4 + instr.imm as usize;
    
    stack[address] = String::from(instr.reg_1.value.as_str());

}


fn get_ordered_data<'a>(data: *const HashMap<String, DotDataVariable>) -> Vec<(&'a String, &'a DotDataVariable)> {
    unsafe {
        let mut ordered_dotdata = data
            .as_ref()
            .unwrap()
            .iter()
            .collect::<Vec<(&String, &DotDataVariable)>>();
        
        ordered_dotdata.sort_unstable_by_key(|t| t.1.v_address);
        ordered_dotdata
    }
}


pub fn get_insert_position<'a>(
    data: &'a mut HashMap<String, DotDataVariable>,
    address: &i64,
) -> &'a mut DotDataVariable {
    let data_raw_pointer = data as *const HashMap<String, DotDataVariable>;
    let ordered_dotdata: Vec<(&String, &DotDataVariable)> = get_ordered_data(data_raw_pointer);

    let mut counter: usize = 1;
    while counter < ordered_dotdata.len() {
        let previous_address: &i64 = &ordered_dotdata[counter-1].1.v_address;
        let current_address: &i64 = &ordered_dotdata[counter].1.v_address;
        
        if *previous_address <= *address && *current_address > *address {
            match data.get(ordered_dotdata[counter-1].0) {
                Some(_value) => { break; },
                None => {  panic!("Address is not allocated with values!"); }
            }
        }
        counter += 1;
    }
    
    let position = data.get_mut(ordered_dotdata[counter-1].0).unwrap();
    position
}


pub fn exec_sw(instr: &mut SType, data: &mut HashMap<String, DotDataVariable>, stack: &mut Stack) {

    if is_stack_operation(instr) { 
        exec_sw_stack(stack, instr); 
        return;
    }

    let re_2_stored_address: &i64 = &instr.reg_2.value.parse::<i64>().unwrap();
    let insert_position = get_insert_position(data, re_2_stored_address);
    
    let index: usize = (&instr.imm / 4) as usize;
    
    match &mut insert_position.v_value {
        Type::Int(inmemory_data) => {
            let requested_position = (re_2_stored_address - insert_position.v_address) / 4;
            inmemory_data[index + requested_position as usize] = instr.reg_1.value.parse().expect(("Not a .word in reg ".to_string() + instr.reg_1.name.as_str()).as_str());
        },
        _ => { panic!("Not a .word!"); }
    }

}


pub fn exec_sb(instr: &mut SType, data: &mut HashMap<String, DotDataVariable>, stack: &mut Stack) {
    let result: i32 = *(&instr.reg_2.value.parse::<i32>().unwrap()) << &instr.imm;
    instr.reg_1.value = result.to_string();
}
