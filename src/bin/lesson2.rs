extern crate sdl2;

use sdl2::video::{Window,WindowPos,OPENGL};
use sdl2::surface::Surface;
use sdl2::timer::delay;

use std::path::Path;

fn main() {
    let context = sdl2::init(sdl2::INIT_EVERYTHING).unwrap();

    let window = match Window::new("lesson 2", WindowPos::PosCentered,
                                   WindowPos::PosCentered, 640, 480, OPENGL) {
        Ok(w)  => w,
        Err(e) => panic!("Failed to create window: {}", e.to_string()),
    };


    let screen = match window.get_surface() {
        Ok(s)  => s,
        Err(e) => panic!("Failed to get screen surface: {}", e.to_string()),
    };

    let image = match Surface::from_bmp(&Path::new("res/hello_world.bmp")) {
        Ok(i)  => i,
        Err(e) => panic!("Failed to create image surface: {}", e.to_string()),
    };

    screen.blit(&image, None, None);
    window.update_surface();

    delay(2000);
    drop(context)
}
