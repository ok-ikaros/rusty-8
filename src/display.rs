use sdl::video;
use sdl::Rect;

use ram::Ram;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct Display {
        gfx: [[u8; WIDTH]; HEIGHT] ,
        draw_flag: bool,
        screen: video::Surface

}

static scale: isize = 20;

impl Display {
        pub fn new() -> Display {
                Display {
                        gfx: [[0; WIDTH]; HEIGHT],   
                        draw_flag: true,
                        screen: video::set_video_mode(64*scale, 32*scale, 8,
                                          &[video::SurfaceFlag::HWSurface],
                                          &[video::VideoFlag::DoubleBuf]).unwrap()

                }
        }

        /* start reading from memory i, and draw that sprite to the gfx */
        pub fn test_draw(&mut self, i_reg: u16, ram: &mut Ram,  x: u8, y: u8, height: u8, vf: &mut u8) {
                println!("drawing sprite at ({}, {})", x, y);
                for y in 0..height {
                        let mut byte = ram.read_byte(i_reg + y as u16);

                        let mut x_coord = x as usize;
                        let mut y_coord = y as usize;
                        
                        for _ in 0..8 {
                                x_coord %= WIDTH;
                                y_coord %= HEIGHT;
                                let prev_bit = self.gfx[y_coord][x_coord];
                                match (byte & 0b1000_0000) >> 7 {
                                        0 => {
                                                
                                                self.gfx[y_coord][x_coord] ^= 0; 
                                                if prev_bit == 1 {
                                                        *vf = 1;
                                                }
                                                else {
                                                        *vf = 0;
                                                }

                                        }
                                        1 => {
                                                self.gfx[y_coord][x_coord] ^= 1; 
                                        }
                                        _ => unreachable!()
                                }
                                x_coord += 1;

                                /* print the next bit */
                                byte = byte << 1;
                        }

                }
                self.draw_flag = true;

        }

        pub fn draw_screen(& mut self) {
                if !self.draw_flag { return }
                let mut pixel: u8;
                let sc = scale as u16;
                let pt = | p: usize| { (p as i16) * (scale as i16) };
                 for y in 0..32 {
                        for x in 0..64 {
                                pixel = if self.gfx[y][x] != 0 { 255 } else { 0 };
                                self.screen.fill_rect(Some(Rect { x: pt(x), y: pt(y), w: sc, h: sc}),
                                video::RGB(pixel, pixel, pixel));
                        }
                }
                self.screen.flip();
                self.draw_flag = false;

        }
        pub fn clear(&mut self) {
                self.gfx = [[0; WIDTH]; HEIGHT];
                self.draw_flag = true;
        }

        pub fn test_print_gfx(&self) {
                for y in 0..HEIGHT {
                        for x in 0..WIDTH {
                                if self.gfx[y][x] == 0 {
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