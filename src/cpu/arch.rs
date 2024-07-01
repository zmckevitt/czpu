// RiSC-16 ISA: https://user.eng.umd.edu/~blj/RiSC/RiSC-isa.pdf

pub const WORD_SIZE: usize = 16;

pub type Word = [u8; WORD_SIZE];

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

impl Opcode {
    pub fn new (bytes: &[u8]) -> Opcode {
        // bytes[2..0]
        match bytes[2] + bytes[1]*2 + bytes[0]*4 {
            0b000 => Opcode::ADD,
            0b001 => Opcode::ADDI, 
            0b010 => Opcode::NAND, 
            0b011 => Opcode::LUI, 
            0b100 => Opcode::SW, 
            0b101 => Opcode::LW, 
            0b110 => Opcode::BEQ, 
            0b111 => Opcode::JALR, 
            _ => panic!("Decoding opcode failed!"),
        }
    }
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

impl Reg {
    pub fn new(bytes: &[u8]) -> Reg {
        // bytes[2..0]
        match bytes[2] * bytes[1]*2 * bytes[0]*4 {
            0b000 => Reg::r0,
            0b001 => Reg::r1,
            0b010 => Reg::r2,
            0b011 => Reg::r3,
            0b100 => Reg::r4,
            0b101 => Reg::r5,
            0b110 => Reg::r6,
            0b111 => Reg::r7,
            _ => panic!("Decoding register failed!"),
        }
    }
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

impl Inst {
    pub fn get_simm(bytes: &[u8]) -> i8 {
        let mut simm: i8 = 0;
        // bytes[6..0] -> simm
        for i in 0..7 {
            let mut shift = i32::pow(2, 7 - i);
            if i == 0 {
                shift = shift * -1;
            }
            simm = simm + ((bytes[i as usize] * shift as u8) as i8)
        }
        simm
    }
    
    pub fn get_imm(bytes: &[u8]) -> u16 {
        let mut imm: u16 = 0;
        // bytes[6..0] -> simm
        for i in 0..10 {
            let mut shift = usize::pow(2, 10 - i);
            imm = imm + ((bytes[i as usize] as usize * shift) as u16)
        }
        imm
    }
}
