extern crate sdl2;

use sdl2::event::{Event, poll_event};
use sdl2::video::{Window,WindowPos,OPENGL};
use sdl2::surface::Surface;

fn main() {
    sdl2::init(sdl2::INIT_EVERYTHING);

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

    'event : loop {
        match poll_event() {
            Event::Quit(_) => break 'event,
            _              => continue,
        }
    }

    sdl2::quit();
}
