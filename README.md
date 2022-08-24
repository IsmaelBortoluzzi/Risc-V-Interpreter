<p align="center">
  <a href="https://doc.rust-lang.org/book/">
    <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/d/d5/Rust_programming_language_black_logo.svg/800px-Rust_programming_language_black_logo.svg.png" alt="Logo" width=72 height=72>
  </a>

  <h3 align="center">Risc-V Interpreter</h3>

  <p align="center">
    Interpreter for Risc-V assembly built with Rust
    <br>
    <a href="https://github.com/IsmaelBortoluzzi/Risc-V-Interpreter/issues/new?template=bug.md">Report bug</a>
    ·
    <a href="https://github.com/IsmaelBortoluzzi/Risc-V-Interpreter/issues/new?template=feature.md&labels=feature">Request feature</a>
  </p>
</p>

## Table of contents

- [Quick start](#quick-start)
- [Exemple Program](#exemple-program)
- [Instructions Supported](#instructions-supported)
- [Ecalls Supported](#ecalls-supported)
- [Not Yet Supported Features](#not-yet-supported-features)
- [What's included](#whats-included)
- [Bugs and feature requests](#bugs-and-feature-requests)
- [Contributing](#contributing)
- [Creators](#creators)
- [Thanks](#thanks)
- [Copyright and license](#copyright-and-license)


## Quick start

The program will take a file path as an arg in the command line

- Install rust [toolchain](https://www.rust-lang.org/tools/install)
- Within project root, on cli, run:
  - cargo run "file.extension"
- Now you should see the result if your assembly code is correct :)

## Exemple Program
    .data
    DIVIDEND: .word 20
    SPACE:.asciz " "
    DIVISOR: .word 7
    ARRAY: .word -118, 8, 7, 99
    RESULT: .word -1
	REMAINDER: .word -1

    .text

    .main:
      # euclidean division algorithm

      la a0, DIVIDEND
      la a1, DIVISOR

      lw a0, 0(a0)
      lw a1, 0(a1)

      jal floorDiv

	  la t0, RESULT
	  sw a0, 0(t0)

	  la t0, REMAINDER
	  sw a1, 0(t0)

      li a7, 10
      ecall


    floorDiv:
        blt a0, zero, normUp
        li t5, 0

    afterNormUp:
        blt a1, zero, normBottom
        li t6, 0

    afterNormBottom:
        li t0, 0

        j while

    normUp:
        sub a0, zero, a0
        li t5, 1

        j afterNormUp

    normBottom:
        sub a1, zero, a1
        li t6, 1

        j afterNormBottom

    while:
        blt a0, a1, endWhile
        sub a0, a0, a1

        addi t0, t0, 1

        j while

    endWhile:
        xor t1, t5, t6
        bgt t1, zero, changeSign
        j endFloorDiv

    changeSign:
        sub t0, zero, t0

    endFloorDiv:
        mv a1, a0
        mv a0, t0

        ret

## Instructions Supported
    // R-type
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Sll,
    Xor,
    Or,
    And,

    // I-type
    Addi,
    Slli,
    Xori,
    Ori,
    Andi,
    Lw,
    La,

    // B-type
    Blt,
    Beq,
    Bgt,

    // S-type
    Sw,

    // J-type
    J,
    Jal,
    Jalr,

## Ecalls Supported
    10 => finish program

## Not Yet Supported Features
- Most of .data, actually only two are supported
  - .word
  - .asciz

## What's included

Basic RI32I and RI32M is covered. Stack is also supported but only using sp reg to access it

```text
interpreter/
└── src/
    ├── dot_data/
    │   ├── data.rs
    │   └── mod.rs
    ├── file_reader/
    │    ├── mod.rs
    │    └── reader.rs
    ├── instructions/
    │    ├── B_type/
    │    │    ├── B_type.rs
    │    │    ├── mod.rs
    │    │    ├── pub_utils.rs
    │    │    └── utils.rs
    │    ├── I_type/
    │    │    ├── I_type.rs
    │    │    ├── load_utils.rs
    │    │    ├── mod.rs  
    │    │    ├── pub_utils.rs
    │    │    └── utils.rs
    │    ├── J_type/
    │    │    ├── J_type.rs
    │    │    ├── mod.rs
    │    │    ├── pub_utils.rs
    │    │    └── utils.rs
    │    ├── R_type/
    │    │    ├── mod.rs
    │    │    ├── pub_utils.rs
    │    │    ├── R_type.rs
    │    │    └── utils.rs
    │    ├── S_type/
    │    │    ├── pub_utils.rs
    │    │    ├── S_type.rs
    │    │    └── utils.rs
    │    ├── instruction.rs
    │    └── mod.rs    
    ├── registers/
    │   ├── mod.rs
    │   └── registers.rs
    ├── stack/
    │   ├── mod.rs
    │   ├── pub_utils.rs
    │   └── stack.rs
    ├── run/
    │    ├── mod.rs
    │    └── run.rs
    ├── .gitignore
    ├── Cargo.lock
    ├── Cargo.toml
    ├── LICENSE.md
    └── README.md
```

## Bugs and feature requests

Have a bug or a feature request? Please first search for existing and closed issues. If your problem or idea is not addressed yet, [please open a new issue](https://github.com/IsmaelBortoluzzi/Risc-V-Interpreter/issues/new).

## Contributing

If you wish to contribute to the project, you can submit your pull request, which will be validated and merged as soon as possible, or open an issue.

## Creators

**Ismael Bortoluzzi**

- <https://github.com/IsmaelBortoluzzi>

## Thanks

Thank you for taking a look at my project!

## Copyright and license

Code released under the [MIT License](https://github.com/IsmaelBortoluzzi/Risc-V-Interpreter/blob/master/LICENSE).

Enjoy