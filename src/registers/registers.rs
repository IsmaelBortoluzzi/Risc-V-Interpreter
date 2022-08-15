use std::collections::HashMap;


#[derive(Debug)]
pub struct Register {
    pub register: String,
    pub name: String,
    pub value: String,
}

impl Register {
    pub fn clone(&self) -> Register {
        Register { register: String::from(&self.register), name: String::from(&self.name), value: String::from(&self.value) }
    }

    pub fn copy_attrs(&mut self, reg: &Register) {
        self.register = String::from(&reg.register);
        self.name = String::from(&reg.name);
        self.value = String::from(&reg.value);
    }
}

pub fn init() -> HashMap<String, Register> {

    let mut registers: Vec<(String, Register)> = Vec::new();
    
    registers.push((String::from("zero"), Register {
        register: String::from("x0"),
        name: String::from("zero"),
        value: String::from("0")
    }));

    registers.push((String::from("ra"), Register {
        register: String::from("x1"),
        name: String::from("ra"),
        value: String::from("0")
    }));

    registers.push((String::from("sp"), Register {
        register: String::from("x2"),
        name: String::from("sp"),
        value: String::from("0")
    }));

    // s

    registers.push((String::from("s0"), Register {
        register: String::from("x8"),
        name: String::from("s0"),
        value: String::from("0")
    }));

    registers.push((String::from("s1"), Register {
        register: String::from("x9"),
        name: String::from("s1"),
        value: String::from("0")
    }));

    registers.push((String::from("s2"), Register {
        register: String::from("x18"),
        name: String::from("s2"),
        value: String::from("0")
    }));

    registers.push((String::from("s3"), Register {
        register: String::from("x19"),
        name: String::from("s3"),
        value: String::from("0")
    }));

    registers.push((String::from("s4"), Register {
        register: String::from("x20"),
        name: String::from("s4"),
        value: String::from("0")
    }));

    registers.push((String::from("s5"), Register {
        register: String::from("x21"),
        name: String::from("s5"),
        value: String::from("0")
    }));


    registers.push((String::from("s6"), Register {
        register: String::from("x22"),
        name: String::from("s6"),
        value: String::from("0")
    }));

    registers.push((String::from("s7"), Register {
        register: String::from("x23"),
        name: String::from("s7"),
        value: String::from("0")
    }));

    registers.push((String::from("s8"), Register {
        register: String::from("x24"),
        name: String::from("s8"),
        value: String::from("0")
    }));

    registers.push((String::from("s9"), Register {
        register: String::from("x25"),
        name: String::from("s9"),
        value: String::from("0")
    }));

    registers.push((String::from("s10"), Register {
        register: String::from("x26"),
        name: String::from("s10"),
        value: String::from("0")
    }));

    registers.push((String::from("s11"), Register {
        register: String::from("x27"),
        name: String::from("s11"),
        value: String::from("0")
    }));

    // a

    registers.push((String::from("a0"), Register {
        register: String::from("x10"),
        name: String::from("a0"),
        value: String::from("0")
    }));

    registers.push((String::from("a1"), Register {
        register: String::from("x11"),
        name: String::from("a1"),
        value: String::from("0")
    }));

    registers.push((String::from("a2"), Register {
        register: String::from("x12"),
        name: String::from("a2"),
        value: String::from("0")
    }));

    registers.push((String::from("a3"), Register {
        register: String::from("x13"),
        name: String::from("a3"),
        value: String::from("0")
    }));

    registers.push((String::from("a4"), Register {
        register: String::from("x14"),
        name: String::from("a4"),
        value: String::from("0")
    }));

    registers.push((String::from("a5"), Register {
        register: String::from("x15"),
        name: String::from("a5"),
        value: String::from("0")
    }));

    registers.push((String::from("a6"), Register {
        register: String::from("x16"),
        name: String::from("a6"),
        value: String::from("0")
    }));

    registers.push((String::from("a7"), Register {
        register: String::from("x17"),
        name: String::from("a7"),
        value: String::from("0")
    }));

    // t

    registers.push((String::from("t0"), Register {
        register: String::from("x5"),
        name: String::from("t0"),
        value: String::from("0")
    }));

    registers.push((String::from("t1"), Register {
        register: String::from("x6"),
        name: String::from("t1"),
        value: String::from("0")
    }));

    registers.push((String::from("t2"), Register {
        register: String::from("x7"),
        name: String::from("t2"),
        value: String::from("0")
    }));

    registers.push((String::from("t3"), Register {
        register: String::from("x28"),
        name: String::from("t3"),
        value: String::from("0")
    }));

    registers.push((String::from("t4"), Register {
        register: String::from("x29"),
        name: String::from("t4"),
        value: String::from("0")
    }));

    registers.push((String::from("t5"), Register {
        register: String::from("x30"),
        name: String::from("t5"),
        value: String::from("0")
    }));

    registers.push((String::from("t6"), Register {
        register: String::from("x31"),
        name: String::from("t6"),
        value: String::from("0")
    }));

    let mut hashmap: HashMap<String, Register> = HashMap::new();

    for i in registers {
        hashmap.insert(i.0, i.1);
    }

    return hashmap;
}
