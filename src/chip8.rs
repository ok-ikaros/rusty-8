use ram::Ram;
use sdl2::VideoSubsystem;
use sdl2::keyboard::Keycode;
use cpu::Cpu;
use cpu;

pub struct Chip8 {
        ram: Ram,
        cpu: Cpu
}

impl Chip8 {
        pub fn new(vid_context: &VideoSubsystem) -> Chip8 {
                Chip8 {
                        ram: Ram::new(),
                        cpu: Cpu::new(&vid_context)
                }
        }

        pub fn load_rom(&mut self, data: &Vec<u8>) {
                /*
                 * Load at the current index, which holds one byte of data
                 * Starting from the offset 
                 */
                for i in 0..data.len() {
                        self.ram.write_byte(cpu::PROG_START + (i as u16), data[i]);
                }
        }

        pub fn execute_opcode(&mut self) {
                self.cpu.execute_opcode(&mut self.ram);
                println!("cpu state: {:?}", self.cpu);
        }

        pub fn press(&mut self, key: Keycode, state: bool) {
                self.cpu.press(key, state);
        }
        pub fn draw_screen(&mut self) {
                self.cpu.draw_screen();
        }
}