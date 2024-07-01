use crate::cpu::arch::*;

const MEM_SIZE: usize = 1024;

pub struct Processor {
    pub pc: usize,
    pub regs: [u8; 8],
    pub mem: [u8; 256],
    pub tick: u64,
}

pub trait Pipeline {
    fn fetch(&self) -> Word; 
    fn decode(&self, word: &Word) -> Inst;
    fn execute(&self, inst: Inst);
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
        // opcode = word[15..13]
        #[inline]
        fn get_opcode(word: &Word) -> Opcode {
            let op: u8 = word[0] >> 5;
            match op {
                0b000 => Opcode::ADD,
                0b001 => Opcode::ADDI,
                0b010 => Opcode::NAND,
                0b011 => Opcode::LUI,
                0b100 => Opcode::LW,
                0b101 => Opcode::SW,
                0b110 => Opcode::BEQ,
                0b111 => Opcode::JALR,
                _ => panic!("Decoding opcode failed!"),
            }
        }

        // reg = word[i..j]
        #[inline]
        fn get_reg(word: &Word, i: usize, j: usize) -> Reg {

            // Need to check if register is on boundary of the two u8s in the Word
            let op: u8 = if(i > 8 && j < 8) {
                word[0] >> j ^ word[1] >> (i - 8)
            } else if (j < 8) {
                word[1] >> j
            } else {
                word[0] >> (j - 8)
            };

            match op {
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

        // simm = word[6..0]
        #[inline]
        fn get_simm(word: &Word) -> i8 {
            let simm: i8 = (word[1] as i8 & 0b11111100u8 as i8) >> 2;
            simm 
        }
        
        // simm = word[9..0]
        #[inline]
        fn get_imm(word: &Word) -> u16 {
            let imm: u16 = word[1] as u16 + word[0] as u16 >> 8;
            imm
        }

        let op = get_opcode(&word);
        let inst: Inst = match op {
            // RRR
            Opcode::ADD | Opcode::NAND => {
                Inst::RRR {
                    // op = word[15..13]
                    op: op, 
                    // ra = word[15..13]
                    ra: get_reg(&word, 12, 10),
                    // rb = word[9..7]
                    rb: get_reg(&word, 9, 7),
                    // Nothing from word[7..3]
                    // rc = word[2..0]
                    rc: get_reg(&word, 2, 0),
                }
            },
            // RRI
            Opcode::ADDI | Opcode::SW | Opcode::LW | Opcode::BEQ | Opcode::JALR => {
                Inst::RRI {
                    op: op,
                    ra: get_reg(&word, 12, 10), 
                    rb: get_reg(&word, 9, 7),
                    simm: get_simm(&word),
                }
            },
            // RI
            Opcode::LUI => {
                Inst::RI {
                    op: op,
                    ra: get_reg(&word, 12, 10),
                    imm: get_imm(&word),
                }
            },
        };
        inst
    }

    fn execute(&self, inst: Inst) {
        match inst {
            Inst::RRR {op, ra, rb, rc}  => println!("RRR"),
            Inst::RRI {op, ra, rb, simm}  => println!("RRI"),
            Inst::RI  {op, ra, imm}  => println!("RI"),
            _ => panic!("Exec err!"),
        }

    }

    fn memory(&self) {

    }

    fn writeback(&self) {

    } 
}
