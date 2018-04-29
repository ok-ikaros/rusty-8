extern crate sdl2;
extern crate rand;

use sdl2::event::Event;

use std::fs::File;
use std::io::Read;
use std::env;
use chip8::Chip8;

mod ram;
mod chip8;
mod cpu;
mod display;
mod keypad;

fn main() {

        let sdl = sdl2::init().unwrap();
        let vid_context = sdl.video().unwrap();
        let mut event_pump = sdl.event_pump().unwrap();

        let args: Vec<String> = env::args().collect();
        let file_name = match args.len() {
                2 => &args[1],
                _ => "roms/INVADERS"
        };
        let mut file = File::open(file_name).unwrap();
        let mut data = Vec::<u8>::new();
        file.read_to_end(&mut data).expect("File not found!");

        let mut chip8 = Chip8::new(&vid_context);
        chip8.load_rom(&data);


        'main: loop {
                for event in event_pump.poll_iter() {
                        match event {
                                Event::Quit { .. } =>  break 'main,
                                Event::KeyDown { keycode: Some(key), .. } => {
                                        chip8.press(key, true);
                                }
                                Event::KeyUp { keycode: Some(key), .. } => {
                                        chip8.press(key, false);
                                }
                                _ => {}
               
                        }       
                }
                chip8.execute_opcode();
                chip8.draw_screen();
        }

}
