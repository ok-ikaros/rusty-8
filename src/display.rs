use std::{thread, time};

use sdl2::pixels::Color;
use sdl2::video::Window;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::VideoSubsystem;

use ram::Ram;

const WIDTH: usize = 64;
const HEIGHT: usize= 32;

pub struct Display {
        gfx: [[u8; WIDTH as usize]; HEIGHT as usize] ,
        draw_flag: bool,
        canvas: Canvas<Window>
}

impl Display {
        pub fn new(video_subsystem: &VideoSubsystem) -> Display {
                let window = video_subsystem.window("rchip-8", 640, 320).build().unwrap();
                let canvas = window.into_canvas().present_vsync().build().unwrap();
                // let window_size = (WIDTH as u32 * 15, HEIGHT as u32 * 15);
                // canvas.set_scale(1.5 as f32, 1.5 as f32).unwrap();    
                // canvas.window_mut().set_size(window_size.0, window_size.1).unwrap();
                Display {
                        gfx: [[0; WIDTH]; HEIGHT],   
                        draw_flag: true,
                        canvas
                }

        }

        /* start reading from memory i, and draw that sprite to the gfx */
        pub fn test_draw(&mut self, i_reg: u16, ram: &mut Ram,  x: u8, y: u8, height: u8, vf: &mut u8) -> bool {
                //println!("drawing sprite at ({}, {})", x, y);
                let mut collision = false;
                for drawn_y in 0..height {
                        let mut byte = ram.read_byte(i_reg + drawn_y as u16);

                        let mut x_coord = x as usize;
                        let mut y_coord = y as usize + drawn_y as usize;
                        
                        for _ in 0..8 {
                                x_coord %= WIDTH;
                                y_coord %= HEIGHT;
                                let prev_bit = self.gfx[y_coord][x_coord];
                                let bit = (byte & 0b1000_0000) >> 7;
                             
                                self.gfx[y_coord][x_coord] ^= bit; 
                                if prev_bit == 1 && self.gfx[y_coord][x_coord] == 0 {
                                        collision = true;
                                }          
                                x_coord += 1;
                                /* print the next bit */
                                byte <<= 1;
                        }

                }
                self.draw_flag = true;
                collision
        }

        pub fn draw_screen(&mut self) {
            self.canvas.clear();
                if !self.draw_flag { return }
                for y in 0..HEIGHT {
                        for x in 0..WIDTH {
                                if self.gfx[y][x] != 0 { 
                                        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
                                } else { 
                                        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
                                };
                                let rect = Rect::new(x as i32 * 10, y as i32 * 10, 10, 10);
                                self.canvas.fill_rect(rect).unwrap();
                                
                        }
                }
                self.canvas.present();
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