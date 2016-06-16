mod cpu;
mod sdl;

use cpu::*;
use sdl::*;

fn main() {
    let mut cpu = Chip8::new(None);
    cpu.load_file("/home/rose/PONG").unwrap();
    cpu.run();
}
