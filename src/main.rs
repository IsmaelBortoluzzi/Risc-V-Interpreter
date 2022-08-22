mod file_reader;
mod registers;
mod dot_data;
mod run;
mod instructions;

use {
    registers::registers::{
        init, 
        Register
    },
    file_reader::reader::{
        file_reader,
        normalize_dotmain,
    },
    std::{
        collections::HashMap,
        env,
    },
    dot_data::data::{DotDataVariable, store_dot_data},
    run::run::{
        run,
        get_all_labels,
    },
};


fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    // lines of the file
    let mut lines: Vec<String> = file_reader();
    normalize_dotmain(&mut lines);
    
    // registers
    let mut registers: HashMap<String, Register> = init();

    // store data defined in .data
    let mut data: HashMap<String, DotDataVariable> = store_dot_data(&mut lines);

    let labels = get_all_labels(&mut lines);

    // execute program line by line
    run(&mut data, &mut registers, &mut lines);

    for x in labels.iter() {
        println!("{}: {:?}", *(x.0), *(x.1));
    }

    println!("\n.DATA");
    for x in data.iter() {
        println!("{}: {:?}", *(x.0), *(x.1));
    }

    println!("\nREGS");
    for x in registers.iter() {
        println!("{}: {:?}", *(x.0), *(x.1));
    }
    println!("\nLINES:");
    for x in lines.iter() {
        println!("{}", *(x));
    }
    
}
