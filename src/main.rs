use std::io::Read;
use std::env;

mod cpu;
mod gui;

// TODO: Check endianness
fn read_image(filename: &str) -> Vec<u8> {
    let mut file = std::fs::File::open(filename).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    buffer
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let image = read_image(&*args[1]); // TODO make sure 1 isn't out of bounds. Can be fixed by using a flag parsing system :)
    let mut cpu = cpu::CPU::new();
    cpu.load_image(0x4, &image);
    cpu.run(0x4);
    gui::gui(cpu).expect("GUI failed to initialize"); // TODO add --no-gui flag
}
