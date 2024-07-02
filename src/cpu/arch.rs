// RiSC-16 ISA: https://user.eng.umd.edu/~blj/RiSC/RiSC-isa.pdf

// pub const WORD_SIZE: usize = 2;

pub type Word = u16; 

pub enum Opcode {
    ADD,
    ADDI,
    NAND,
    LUI,
    SW,
    LW,
    BEQ,
    JALR,
}

#[allow(non_camel_case_types)]
pub enum Reg {
    r0,
    r1,
    r2,
    r3,
    r4,
    r5,
    r6,
    r7,
}

pub enum Inst {
    RRR {
        op: Opcode,
        ra: Reg,
        rb: Reg,
        rc: Reg,
    },
    RRI {
        op: Opcode, 
        ra: Reg,
        rb: Reg,
        simm: i8,
    },
    RI {
        op: Opcode,
        ra: Reg,
        imm: u16,
    },
}
