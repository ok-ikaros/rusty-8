use ram::Ram;
use sdl2::keyboard::Keycode;
use std::fmt;
use display::Display;
use keypad::Keypad;
use sdl2::VideoSubsystem;
use rand;
use rand::distributions::{IndependentSample, Range};
use std::time;
use std::time::Duration;


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
        delay_set: time::Instant,
        sound_timer: u8,
        rng: rand::ThreadRng

}

impl Cpu {
        pub fn new(vid_context: &VideoSubsystem) -> Cpu {
                Cpu {
                        display: Display::new(&vid_context),
                        keypad: Keypad::new(),
                        v: [0; 16],
                        pc: PROG_START,
                        i: 0,
                        stack: Vec::<u16>::new(),
                        //sp: 0,
                        prev_pc: 0,
                        delay_timer: 0,
                        delay_set: time::Instant::now(),
                        sound_timer: 0,
                        rng: rand::thread_rng()

                }

        }
        pub fn execute_opcode(&mut self, ram: &mut Ram) {
                
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

                // if self.prev_pc == self.pc {
                //         panic!("increment the pc");
                // }

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
                        0x5 => {
                                if self.v[x as usize] == self.v[y as usize] {
                                        self.pc += 4;
                                }
                                else {
                                        self.pc += 2;
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
                                        1 => {
                                                self.v[x as usize] |= self.v[y as usize];
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
                                        7 => {
                                                if vy > vx {
                                                        self.v[0xF] = 1;
                                                } else {
                                                        self.v[0xF] = 0;
                                                }
                                                self.v[x as usize] = vy - vx;

                                        },
                                        0xE => {
                                                self.v[0xF] = self.v[x as usize] >> 7;
                                                self.v[x as usize] <<= 1;
                                        }

                                        _ => panic!("Unimplemented opcode {:#X}:{:#X}", self.pc, opcode)
                                };
                                self.pc += 2;
                        },
                        0x9 => {
                                if self.v[x as usize] != self.v[y as usize] {
                                        self.pc += 4;
                                } else {
                                        self.pc += 2;
                                }
            
                        },
                        0xA => {
                                /* i = nnn */
                                self.i = nnn;
                                self.pc += 2;
                        },
                        0xB => {
                                self.v[x as usize] as u16 + nnn;

                        },
                        0xC => {
                                let interval = Range::new(0, 255);
                                let number = interval.ind_sample(&mut self.rng);
                                self.v[x as usize] = number & nn;
                                self.pc += 2;

                        },
                        0xD => {
                                /* draw sprite */
                                let collision = self.display.test_draw(self.i, ram, self.v[x as usize], self.v[y as usize], n, &mut self.v[0xF]);
                                if collision {
                                    self.v[0xF] = 1;
                                }
                                else {
                                    self.v[0xF] = 0;
                                }
                                //self.display.test_draw(self.i, ram, x, y, n, &mut self.v[0xF]);
                               // self.display.test_print_gfx();
                                self.pc += 2;

                        },
                        0xE => {
                                match nn {
                                        0xA1 => {
                                                /* if key() != vx, skip  */
                                                let keycode = self.v[x as usize];
                                                if !self.keypad.pressed(keycode as usize) {
                                                        self.pc += 4
                                                } else {
                                                        self.pc += 2
                                                }
 
                                        },
                                        0x9E => {
                                                /* if key() == vx, skip */
                                                let keycode = self.v[x as usize];
                                                if self.keypad.pressed(keycode as usize) {
                                                        self.pc += 4
                                                } else {
                                                        self.pc += 2
                                                }

                                        }
                                        _ => panic!("Unimplemented opcode {:#X}:{:#X}", self.pc, opcode)
                                };
                        },
                        0xF => {
                                match nn {
                                        0x07 => {
                                                self.v[x as usize] = self.get_delay();
                                                //self.v[x as usize] = self.delay_timer;
                                        },
                                        0x15 => {
                                                let vx = self.v[x as usize];
                                                self.set_delay(vx);
                                                //self.delay_timer = self.v[x as usize];
                                        },
                                        0x18 => {
                                                self.sound_timer = self.v[x as usize];
                                        }
                                        0x0A => {
                                                self.wait_keypress(x);
                                        },
                                        0x1E => {
                                                let vx = self.v[x as usize];
                                                self.i += vx as u16;
                                        },
                                        0x29 => {
                                                /* have to multiply by 5 because each sprite has 5 lines */
                                                self.i = self.v[x as usize] as u16 * 5;
                                        },
                                        0x33 => {
                                                let vx = self.v[x as usize];
                                                ram.write_byte(self.i, vx / 100);
                                                ram.write_byte(self.i + 1, (vx % 100) / 10);
                                                ram.write_byte(self.i + 2, vx % 10);

                                        },
                                        0x55 => {
                                                for index in 0..x + 1 {
                                                        let value = self.v[x as usize];
                                                        ram.write_byte(self.i + index as u16, value);
                                                }
                                                self.i += x as u16 + 1;
                                                self.pc += 2;
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

        pub fn set_delay(&mut self, val: u8) {
            self.delay_set = time::Instant::now();
            self.delay_timer = val;
        }

        pub fn get_delay(&self) -> u8 {
            let diff = time::Instant::now() - self.delay_set;
            let ms = diff.get_millis();
            /* These ticks are 60 hz */
            let ticks = ms / 16;
            if ticks >= self.delay_timer as u64 {
                0
            } else {
                self.delay_timer - ticks as u8
            }
        }
        pub fn press(&mut self, key: Keycode, state: bool) {
                self.keypad.press(key, state);
        }

        fn wait_keypress(&mut self, x: u8) {
                for i in 0..16 {
                        if self.keypad.pressed(i as usize) {
                                self.v[x as usize] = i;
                                break;
                        }
                }
                self.pc -= 2;
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

trait Milliseconds {
    fn get_millis(&self) -> u64;
}

impl Milliseconds for Duration {
    fn get_millis(&self) -> u64 {
        let nanos = self.subsec_nanos() as u64;
        let ms = (1000*1000*1000 * self.as_secs() + nanos) /(1000 * 1000);
        ms
    }
}

impl fmt::Debug for Cpu {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "\npc: {:#X}\n", self.pc)?;
                write!(f, "vx: ")?;
                for item in self.v.iter() {
                        write!(f, "{:#X} ", *item)?;
                }
                write!(f, "\n")?;
                write!(f, "i: {:#X}\n", self.i)?;
                write!(f, "\ndelay_timer: {:?}\n", self.delay_timer)

        }
}