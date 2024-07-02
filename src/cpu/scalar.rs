use crate::cpu::arch::*;

const MEM_SIZE: usize = 1024;

pub struct Processor {
    pub pc: usize,
    pub regs: [u16; 8],
    pub mem: [u16; 256],
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
        assert!(self.pc + 1 < MEM_SIZE, "Trying to fetch outside memory!");
        // Convert [u8] -> [u8; 16]
        self.mem[self.pc]
    }

    fn decode(&self, word: &Word) -> Inst {
        // opcode = word[15..13]
        #[inline]
        fn get_opcode(word: &Word) -> Opcode {
            let op: u8 = (word >> 12) as u8;
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
            let reg = (word >> j) & (0b111 as u16);
            match reg {
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
            let simm = ((word & (0b01111111u16)) as i8) << 1 >> 1;
            simm 
        }
        
        // imm = word[9..0]
        #[inline]
        fn get_imm(word: &Word) -> u16 {
            let imm: u16 = (((word >> 8) & 0b00000011) << 8) + (word & 0b11111111u16);
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
                    // op = word[15..13]
                    op: op,
                    // ra = word[15..13]
                    ra: get_reg(&word, 12, 10),
                    // rb = word[9..7]
                    rb: get_reg(&word, 9, 7),
                    // simm = word[6..0]
                    simm: get_simm(&word),
                }
            },
            // RI
            Opcode::LUI => {
                Inst::RI {
                    // op = word[15..13]
                    op: op,
                    // ra = word[15..13]
                    ra: get_reg(&word, 12, 10),
                    // imm = word[9..0]
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
