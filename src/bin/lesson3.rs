extern crate sdl2;

use sdl2::event::Event;
use sdl2::video::{Window,WindowPos,OPENGL};
use sdl2::surface::Surface;

use std::path::Path;

fn main() {
    let context = sdl2::init(sdl2::INIT_EVERYTHING).unwrap();

    let window = match Window::new("lesson 3", WindowPos::PosCentered,
                                   WindowPos::PosCentered, 640, 480, OPENGL) {
        Ok(w)  => w,
        Err(e) => panic!("Failed to create window: {}", e.to_string()),
    };


    let screen = match window.get_surface() {
        Ok(s)  => s,
        Err(e) => panic!("Failed to get screen surface: {}", e.to_string()),
    };

    let image = match Surface::from_bmp(&Path::new("res/x.bmp")) {
        Ok(i)  => i,
        Err(e) => panic!("Failed to create image surface: {}", e.to_string()),
    };

    screen.blit(&image, None, None);
    window.update_surface();

    let mut event_pump = context.event_pump();
    let mut event_iter = event_pump.poll_iter();
    loop {
        if let Some(evt) = event_iter.next() {
            match evt {
                Event::Quit{..} => break,
                _              => {},
            }
        }
    }

}
