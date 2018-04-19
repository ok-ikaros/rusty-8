extern crate sdl2;

use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::fs::File;
use std::io::Read;
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
        let mut file = File::open("roms/INVADERS").unwrap();

        let mut data = Vec::<u8>::new();
        file.read_to_end(&mut data);
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
