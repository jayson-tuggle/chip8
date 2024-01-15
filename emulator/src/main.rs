mod drivers;
mod processor;

use crate::processor::Processor;

fn main() {
    let mut chip8 = Processor::new();
    chip8.load_rom("./roms/programs/IBM Logo.ch8".to_string());
}
