use std::io::Read;
use std::env;

mod cpu;

fn read_image(filename: &str) -> Vec<u8> {
    let mut file = std::fs::File::open(filename).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    buffer
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let image = read_image(&*args[1]);
    let mut cpu = cpu::CPU::new();
    cpu.load_image(0x0, &image);
    cpu.run(0x0);
}
