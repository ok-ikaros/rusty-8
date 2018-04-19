use ram::Ram;
use std::fmt;
use display::Display;
use keypad::Keypad;

pub const PROG_START: u16 = 0x200;

pub struct Cpu {
        /* General purpose. X is a hex digit from 0 - F */
        v: [u8; 16],
        /* program counter */
        pc: u16,
        /* stores memory addresses - usually only 12 leftmost bits used */
        i: u16,
        stack: Vec<u16>, // TODO CHANGE TO VECTOR
        /* stack pointer */
        //sp: usize,
        prev_pc: u16,
        display: Display,
        keypad: Keypad,
        delay_timer: u8,
        sound_timer: u8

}

impl Cpu {
        pub fn new() -> Cpu {
                Cpu {
                        display: Display::new(),
                        keypad: Keypad::new(),
                        v: [0; 16],
                        pc: PROG_START,
                        i: 0,
                        stack: Vec::<u16>::new(),
                        //sp: 0,
                        prev_pc: 0,
                        delay_timer: 0,
                        sound_timer: 0

                }

        }
        pub fn execute_opcode(&mut self, ram: &mut Ram) {
                self.tick();
                let first_byte = ram.read_byte(self.pc) as u16;
                let second_byte= ram.read_byte(self.pc + 1) as u16;

                /* 
                 * Shifting lo over to the left 8 bits, adding 8 zeroes
                 * Use bitwise or to merge the two 
                 */
                let opcode: u16 = (first_byte << 8) | second_byte;
                println!("opcode: {:#X}:{:#X}: hi:{:#X} lo:{:#X}", self.pc, opcode, first_byte, second_byte);
                let nnn = opcode & 0x0FFF;
                let nn = (opcode & 0x0FF) as u8;

                /* n is also called nibble for some reason */
                let n = (opcode & 0x00F) as u8;
                let x = ((opcode & 0x0F00) >> 8) as u8;
                let y = ((opcode & 0x00F0) >> 4) as u8;
                println!("nnn = {:?}, nn = {:?}, n = {:?}, x = {:?}, y = {:?}", nnn, nn, n, x, y);

                if self.prev_pc == self.pc {
                        panic!("increment the pc");
                }

                self.prev_pc = self.pc;
                match (opcode & 0xF000) >> 12 {
                        0x0 => {
                                match nn {
                                        0xE0 => {
                                                self.display.clear();
                                                self.pc += 2;
                                        },
                                        0xEE => {
                                                /* return from subroutine */
                                                // self.sp -= 1;
                                                // self.pc = self.stack[self.sp];
                                                let addr = self.stack.pop().unwrap();
                                                self.pc = addr;

                                                
                                        },
                                        _ => {
                                                panic!("Unimplemented opcode {:#X}:{:#X}", self.pc, opcode)

                                        }
                                }

                        },
                        0x1 => {
                                /* goto nnn */
                                self.pc = nnn;

                        },
                        0x2 => {
                                /* calls subroutine at nnn */
                                // self.stack[self.sp] = self.pc + 2;
                                // self.sp += 1;
                                // self.pc = nnn;
                                self.stack.push(self.pc + 2);
                                self.pc = nnn;
                        }
                        0x3 => {
                                /* if vx == nn */
                                if self.v[x as usize] == nn {
                                        self.pc += 4;   
                                }
                                else {
                                        self.pc += 2;
                                }
                        },
                        0x4 => {
                                if self.v[x as usize] != nn {
                                        self.pc += 4
                                }
                                else {
                                        self.pc += 2
                                }
                        },
                        0x6 => {
                                /* vx = nn */
                                self.v[x as usize] = nn;
                                self.pc += 2;

                        },
                        0x7 => {
                                /* vx += nn */
                                let vx = self.v[x as usize];
                                self.v[x as usize] = vx.wrapping_add(nn);
                                self.pc += 2;
                        },
                        0x8 => {
                                let vx = self.v[x as usize];
                                let vy = self.v[y as usize];
                                match n {
                                        0 => {
                                                /* vx = vy */
                                                self.v[x as usize] = self.v[y as usize];

                                        },
                                        2 => {
                                                /* vx = vx & vy */
                                                self.v[x as usize] = vx & vy;

                                        },
                                        3 => {
                                                /* vx = vx ^ vy */
                                                self.v[x as usize] = vx ^ vy;
                                                self.pc += 2;

                                        },
                                        4 => {
                                                /* vx = vx += vy */
                                        
                                                let sum: u16 = vx as u16 + vy as u16;
                                                self.v[x as usize] = sum as u8;
                                                if sum > 0xFF {
                                                        self.v[0xF] = 1;
                                                }
                                                // else {
                                                //         self.v[0xF] = 0;
                                                // }
                                        },
                                        5 => {
                                                
                                                /* vx = vx -= vy */
                                                if vx > vy {
                                                        self.v[0xF] = 1;
                                                } else {
                                                        self.v[0xF] = 0;
                                                }
                                                self.v[x as usize] = vx - vy;
                                        },
                                        6 => {
                                                 
                                                 /* Shifts VY right by one and copies the result to VX. 
                                                 * VF is set to the value of the least significant bit 
                                                 * of VY before the shift */
                                                 
                                                self.v[0xF] = vx & 0x1;
                                                self.v[x as usize] >>= 1;
                                                // self.v[y as usize] >>= 1;

                                        },


                                        _ => panic!("Unimplemented opcode {:#X}:{:#X}", self.pc, opcode)
                                };
                                self.pc += 2;
                        },
                        0xD => {
                                /* draw sprite */
                                self.display.test_draw(self.i, ram, self.v[x as usize], self.v[y as usize], n, &mut self.v[0xF]);
                                //self.display.test_draw(self.i, ram, x, y, n, &mut self.v[0xF]);
                                self.display.test_print_gfx();
                                self.pc += 2;

                        },
                        0xE => {
                                match nn {
                                        0xA1 => {
                                                /* if key() != vx, skip  */
                                                let keycode = self.v[x as usize];
                                                if !self.keypad.key_is_pressed(keycode) {
                                                        self.pc += 4
                                                } else {
                                                        self.pc += 2
                                                }
 
                                        },
                                        0x9E => {
                                                /* if key() == vx, skip */
                                                let keycode = self.v[x as usize];
                                                if self.keypad.key_is_pressed(keycode) {
                                                        self.pc += 4
                                                } else {
                                                        self.pc += 2
                                                }

                                        }
                                        _ => panic!("Unimplemented opcode {:#X}:{:#X}", self.pc, opcode)
                                };
                        },
                        0xA => {
                                /* i = nnn */
                                self.i = nnn;
                                self.pc += 2;
                        },
                        0xF => {
                                match nn {
                                        0x07 => {
                                                self.v[x as usize] = self.delay_timer;
                                        },
                                        0x15 => {
                                                self.delay_timer = self.v[x as usize];
                                        },
                                        0x18 => {
                                                self.sound_timer = self.v[x as usize];
                                        }
                                        0x0A => {
                                               
                                        }
                                        0x1E => {
                                                let vx = self.v[x as usize];
                                                self.i += vx as u16;
                                        },
                                        0x65 => {
                                                for i in 0..x + 1 {
                                                        self.v[i as usize] = ram.read_byte(self.i + i as u16)
                                                }
                                                self.i = x as u16 + 1;
                                        }

                                         _ => panic!("Unimplemented opcode {:#X}:{:#X}", self.pc, opcode)
                                }
                                self.pc += 2;     
                        }
                        _ => panic!("Unimplemented {:#X}:{:#X}", self.pc, opcode)
                }
                

        }

        pub fn draw_screen(&mut self) {
                self.display.draw_screen();
        }

        pub fn tick(&mut self) {
                if self.delay_timer > 0 {
                        self.delay_timer -= 1;
                }
        }
}

impl fmt::Debug for Cpu {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "\npc: {:#X}\n", self.pc);
                write!(f, "vx: ");
                for item in self.v.iter() {
                        write!(f, "{:#X} ", *item);
                }
                write!(f, "\n");
                write!(f, "i: {:#X}\n", self.i);
                write!(f, "\ndelay_timer: {:?}\n", self.delay_timer)

        }
}