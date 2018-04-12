use ram::Ram;

const WIDTH: usize = 64;
const HEIGHT: usize = 64;

pub struct Display {
        screen: [[u8; WIDTH]; HEIGHT] 
}

impl Display {
        pub fn new() -> Display {
                Display {
                        screen: [[0; WIDTH]; HEIGHT],   

                }
        }

        /* start reading from memory i, and draw that sprite to the screen */
        pub fn test_draw(&mut self, i_reg: u16, ram: &mut Ram,  x: u8, y: u8, height: u8, vf: &mut u8) {
                println!("drawing sprite at ({}, {})", x, y);
                for y in 0..height {
                        let mut byte = ram.read_byte(i_reg + y as u16);

                        let mut x_coord = x as usize;
                        let y_coord = y as usize;
                        
                        for _ in 0..8 {
                                match (byte & 0b1000_0000) >> 7 {
                                        0 => {
                                                if self.screen[y_coord][x_coord] == 1 {
                                                        *vf = 1;
                                                }
                                                else {
                                                        *vf = 0;
                                                }
                                                self.screen[y_coord][x_coord] = 0; 

                                        }
                                        1 => {
                                                self.screen[y_coord][x_coord] = 1; 
                                        }
                                        _ => unreachable!()
                                }
                                x_coord += 1;

                                /* print the next bit */
                                byte = byte << 1;
                        }

                }
        }
        pub fn clear(&mut self) {
                self.screen = [[0; WIDTH]; HEIGHT];
        }

        pub fn test_print_screen(&self) {
                for y in 0..HEIGHT {
                        for x in 0..WIDTH {
                                if self.screen[y][x] == 0 {
                                        print!("_")
                                }
                                else {
                                        print!("#")
                                }
                        }
                        print!("\n")
                }

        }
}