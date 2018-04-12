use ram::Ram;
use std::fmt;

pub const PROG_START: u16 = 0x200;

pub struct Cpu {
        /* General purpose. X is a hex digit from 0 - F */
        vx: [u8; 16],
        pc: u16,
        /* stores memory addresses - usually only 12 leftmost bits used */
        i: u16,
        prev_pc: u16
}

impl Cpu {
        pub fn new() -> Cpu {
                Cpu {
                        vx: [0; 16],
                        pc: PROG_START,
                        i: 0,
                        prev_pc: 0

                }

        }
        pub fn run_instruction(&mut self, ram: &mut Ram) {
                let hi = ram.read_byte(self.pc) as u16;
                let lo = ram.read_byte(self.pc + 1) as u16;
                /* 
                 * Shifting lo over to the left 8 bits, adding 8 zeroes
                 * Use bitwise or to merge the two 
                 */
                let instruction: u16 = (hi << 8) | lo;
                println!("Instruction read: {:#X}: hi:{:#X} lo:{:#X}", instruction, hi, lo);
                let nnn = instruction & 0x0FFF;
                let nn = (instruction & 0x0FF) as u8;
                let n = instruction & 0x00F;
                let x = (instruction & 0x0F00) >> 8;
                let y = (instruction & 0x00F0) >> 4;
                println!("nnn = {:?}, nn = {:?}, n = {:?}, x = {:?}, y = {:?}", nnn, nn, n, x, y);

                if self.prev_pc == self.pc {
                        panic!("increment the pc");
                }

                self.prev_pc = self.pc;
                match (instruction & 0xF000) >> 12 {
                        0x1 => {
                                /* goto nnn */
                                self.pc = nnn;

                        }
                        0x6 => {
                                /* vx = nn */
                                self.write_reg_vx(x, nn);
                                self.pc += 2;

                        }
                        0xA => {
                                /* i = nnn */
                                self.i = nnn;
                        }
                        _ => panic!("Unrecognized instruction {:#X}:{:#X}", self.pc, instruction)
                }
                

        }

        /* nn has to be a u8 because the register values are 8 bits */
        pub fn write_reg_vx(&mut self, index: u16, value: u8) {
                self.vx[index as usize] = value;
        }

        pub fn read_reg_vx(&mut self, index: u16) -> u8 {
                self.vx[index as usize]

        }
}

impl fmt::Debug for Cpu {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "pc: {:#X}\n", self.pc);
                write!(f, "vx: ");
                for item in self.vx.iter() {
                        write!(f, "{:#X} ", *item);
                }
                write!(f, "\n");
                write!(f, "i: {:#X}\n", self.i)
        }
}