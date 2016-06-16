extern crate sdl2;
use self::sdl2::*;
use cpu::Render;

impl Render for Sdl {
    fn clear(&self, screen: &mut [[bool; 64]; 32]) {
        *screen = [[false; 64]; 32];
    }
}
