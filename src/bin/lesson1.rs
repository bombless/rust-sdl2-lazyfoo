extern crate sdl2;

use sdl2::video::{Window,WindowPos,OPENGL};
use sdl2::timer::delay;
use sdl2::pixels::Color;

fn main() {
    let context = sdl2::init(sdl2::INIT_EVERYTHING).unwrap();

    let window = match Window::new("lesson 1", WindowPos::PosCentered,
                                   WindowPos::PosCentered, 640, 480, OPENGL) {
        Ok(w)  => w,
        Err(e) => panic!("Failed to create window: {}", e.to_string()),
    };


    let mut screen = match window.get_surface() {
        Ok(s)  => s,
        Err(e) => panic!("Failed to get screen surface: {}", e.to_string()),
    };

    screen.fill_rect(None, Color::RGB(255, 255, 255)).unwrap();
    window.update_surface();

    delay(2000);
}
