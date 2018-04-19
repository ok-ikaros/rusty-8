extern crate sdl;
use sdl::event::Event;

use std::fs::File;
use std::io::Read;
use chip8::Chip8;



mod ram;
mod chip8;
mod cpu;
mod display;
mod keypad;

fn main() {
        let mut file = File::open("roms/INVADERS").unwrap();

        let mut data = Vec::<u8>::new();
        file.read_to_end(&mut data);
        let mut chip8 = Chip8::new();
        chip8.load_rom(&data);

        sdl::init(&[sdl::InitFlag::Video, sdl::InitFlag::Audio, sdl::InitFlag::Timer]);

        // loop {
        //         chip8.execute_opcode();
        // }

        'main : loop {
                'event : loop {
                        match sdl::event::poll_event() {
                                Event::Quit                  => break 'main,
                                Event::None                  => break 'event,
                                Event::Key(key, state, _, _) => chip8.keypad.press(key, state),
                                 _                           => {}
                        }                           
                }

                chip8.execute_opcode();
                chip8.draw_screen();
        }

        sdl::quit();

}
