use crate::cpu::arch::*;

const MEM_SIZE: usize = 1024;

pub struct Processor {
    pub pc: usize,
    pub mem: [u8; 256],
    pub tick: u64,
}

pub trait Pipeline {
    fn fetch(&self) -> Word; 
    fn decode(&self, word: &Word) -> Inst;
    fn execute(&self);
    fn memory(&self);
    fn writeback(&self);
}

impl Pipeline for Processor {
    fn fetch(&self) -> Word {
        assert!(self.pc + WORD_SIZE < MEM_SIZE, "Trying to fetch outside memory!");
        // Convert [u8] -> [u8; 16]
        self.mem[self.pc..self.pc+WORD_SIZE].try_into().unwrap()
    }

    fn decode(&self, word: &Word) -> Inst {
        let op = word[13] + word[14]*2 + word[15]*4;
        let inst: Inst = match op {
            // RRR
            0b000 | 0b010 => {
                Inst::RRR {
                    op: Opcode::new(&word[13..16]), 
                    ra: Reg::new(&word[10..13]), 
                    rb: Reg::new(&word[7..10]),
                    // Nothing from word[3:7]
                    rc: Reg::new(&word[0..3]),
                }
            },
            // RRI
            0b001 | 0b101 | 0b100 | 0b110 | 111 => {
                Inst::RRI {
                    op: Opcode::new(&word[13..16]), 
                    ra: Reg::new(&word[10..13]), 
                    rb: Reg::new(&word[7..10]),
                    simm: Inst::get_simm(&word[0..7]),
                }
            },
            // RI
            0b011 => {
                Inst::RI {
                    op: Opcode::new(&word[13..16]),
                    ra: Reg::new(&word[10..13]),
                    imm: Inst::get_imm(&word[0..10]),
                }
            },
            _ => panic!("Decode error!"),
        };
        inst
    }

    fn execute(&self) {

    }

    fn memory(&self) {

    }

    fn writeback(&self) {

    } 
}
