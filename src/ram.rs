pub struct Ram {
        mem: [u8; 4096],
}

impl Ram {
        pub fn new() -> Ram {
                let mut ram = Ram {
                        mem: [0; 4096]
                };

                /* 16 sprites. Each sprite is an array of 5 bytes. */ 
                let fontset: [u8; 80] = [ 0xF0, 0x90, 0x90, 0x90, 0xF0, 
                                          0x20, 0x60, 0x20, 0x20, 0x70,
                                          0xF0, 0x10, 0xF0, 0x80, 0xF0, 
                                          0xF0, 0x10, 0xF0, 0x10, 0xF0,
                                          0x90, 0x90, 0xF0, 0x10, 0x10,
                                          0xF0, 0x80, 0xF0, 0x10, 0xF0, 
                                          0xF0, 0x80, 0xF0, 0x90, 0xF0, 
                                          0xF0, 0x10, 0x20, 0x40, 0x40, 
                                          0xF0, 0x90, 0xF0, 0x90, 0xF0, 
                                          0xF0, 0x90, 0xF0, 0x10, 0xF0, 
                                          0xF0, 0x90, 0xF0, 0x90, 0x90,
                                          0xE0, 0x90, 0xE0, 0x90, 0xE0, 
                                          0xF0, 0x80, 0x80, 0x80, 0xF0, 
                                          0xE0, 0x90, 0x90, 0x90, 0xE0, 
                                          0xF0, 0x80, 0xF0, 0x80, 0xF0,
                                          0xF0, 0x80, 0xF0, 0x80, 0x80, 
                                        ];
                for i in 0..80 { 
                        ram.mem[i] = fontset[i]; 
                }

                ram
        }

        pub fn write_byte(&mut self, address: u16, value: u8) {
                self.mem[address as usize] = value;

        }

        pub fn read_byte(&mut self, address: u16) -> u8 {
                       
                       
                self.mem[address as usize]
        }
}