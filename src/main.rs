use std::env;

enum Registers {
    R_R0 = 0,
    R_R1,
    R_R2,
    R_R3,
    R_R4,
    R_R5,
    R_R6,
    R_R7,
    R_PC,
    R_COND,
    R_COUNT
}

enum Opcode {
    OP_BR = 0, /* branch */
    OP_ADD,    /* add  */
    OP_LD,     /* load */
    OP_ST,     /* store */
    OP_JSR,    /* jump register */
    OP_AND,    /* bitwise and */
    OP_LDR,    /* load register */
    OP_STR,    /* store register */
    OP_RTI,    /* unused */
    OP_NOT,    /* bitwise not */
    OP_LDI,    /* load indirect */
    OP_STI,    /* store indirect */
    OP_JMP,    /* jump */
    OP_RES,    /* reserved (unused) */
    OP_LEA,    /* load effective address */
    OP_TRAP    /* execute trap */
}

#[derive(Debug)]
enum Flag {
    FL_POS = 1 << 0,
    FL_ZRO = 1 << 1,
    FL_NEG = 1 << 2
}

fn main() {
    // 65536 locations
    let mut memory: [u16; std::u16::MAX as usize] = [0; std::u16::MAX as usize];

    // init registers
    let mut reg: [u16; Registers::R_COUNT as usize] = [0; Registers::R_COUNT as usize];

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    // TODO: read image to memory

    // set the PC to the starting position
    // 0x3000 is the default
    let pc_start: u16 = 0x3000;
    reg[Registers::R_PC as usize] = pc_start;

    let mut running: bool = true;
    while(running) {
        // temporary instruction
        reg[0] = 0;
        reg[1] = 0;
        reg[2] = 0;
        let instr: u16 = 0b0001_0000_0100_0010;
        // TODO: fetch opcode
        // temporary opcode
        let opcode: Opcode = Opcode::OP_ADD;

        match opcode {
            Opcode::OP_ADD => {
                reg = add(reg, instr);
                println!("{}", reg[0]);
                println!("aaa {}", reg[Registers::R_COND as usize]);
            },
            Opcode::OP_AND => {
                // TODO: implement instruction
            },
            Opcode::OP_NOT => {
                // TODO: implement instruction
            },
            Opcode::OP_BR => {
                // TODO: implement instruction
            },
            Opcode::OP_JMP => {
                // TODO: implement instruction 
            },
            Opcode::OP_JSR => {
                // TODO: implement instruction
            },
            Opcode::OP_LD => {
                // TODO: implement instruction
            },
            Opcode::OP_LDI => {
                // TODO: implement instruction
            },
            Opcode::OP_LDR => {
                // TODO: implement instruction
            },
            Opcode::OP_LEA => {
                // TODO: implement instruction
            },
            Opcode::OP_ST => {
                // TODO: implement instruction
            },
            Opcode::OP_STI => {
                // TODO: implement instruction
            },
            Opcode::OP_STR => {
                // TODO: implement instruction
            },
            Opcode::OP_TRAP => {
                // TODO: implement instruction
            },
            _ => {
                // TODO: implement instruction
            }
        }

        running = false;
    }
}

fn add(mut reg: [u16; Registers::R_COUNT as usize], instr: u16) -> [u16; Registers::R_COUNT as usize] {
    // destination register (DR)
    let r0: u16 = (instr >> 9) & 0b0111;
    // first operand (SR1)
    let r1: u16 = (instr >> 6) & 0b0111;
    // whether we are in immediate mode
    let is_imm_mode: bool = (instr >> 5) & 0b0001 == 1;

    if is_imm_mode {
        let imm5: u16 = sign_extend(instr & 0b0001_1111, 5);
        reg[r0 as usize] = reg[r1 as usize] + imm5;
    } else {
        let r2 = instr & 0b0111;
        reg[r0 as usize] = reg[r1 as usize] + reg[r2 as usize];
    }
    reg = update_flags(reg, r0);
    return reg;
}

fn sign_extend(mut x: u16, bit_count: u16) -> u16 {
    if x >> (bit_count - 1) & 1 == 1 {
        x |= (0xFFFF << bit_count);
    }
    return x;
}

fn update_flags(mut reg: [u16; Registers::R_COUNT as usize], r: u16) -> [u16; Registers::R_COUNT as usize] {
    if reg[r as usize] == 0 {
        reg[Registers::R_COND as usize] = Flag::FL_ZRO as u16;
    } else if reg[r as usize] >> 15 == 1 {
        reg[Registers::R_COND as usize] = Flag::FL_NEG as u16;
    } else {
        reg[Registers::R_COND as usize] = Flag::FL_POS as u16;
    }
    return reg;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn add_returns_correct_result_with_positive_condition() {
        let mut reg: [u16; Registers::R_COUNT as usize] = [0; Registers::R_COUNT as usize];
        reg[0] = 0;
        reg[1] = 1;
        reg[2] = 2;
        let instr: u16 = 0b0001_0000_0100_0010;
        reg = add(reg, instr);
        assert_eq!(reg[0], 3);
        assert_eq!(reg[Registers::R_COND as usize], Flag::FL_POS as u16);
    }
}

