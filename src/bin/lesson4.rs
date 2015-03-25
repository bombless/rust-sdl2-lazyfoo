extern crate sdl2;

use sdl2::event::Event;
use sdl2::video::{Window,WindowPos,OPENGL};
use sdl2::surface::Surface;
use sdl2::keycode::KeyCode;

use std::path::Path;

fn load_image(filename: &str) -> Surface {
    match Surface::from_bmp(&Path::new(filename)) {
        Ok(s) => return s,
        Err(e) => panic!("Failed to load image surface: {}", e.to_string()),
    }
}

fn main() {
    let context = sdl2::init(sdl2::INIT_EVERYTHING).unwrap();

    let window = match Window::new("lesson 4", WindowPos::PosCentered,
                                   WindowPos::PosCentered, 640, 480, OPENGL) {
        Ok(w)  => w,
        Err(e) => panic!("Failed to create window: {}", e.to_string()),
    };

    let screen = match window.get_surface() {
        Ok(s)  => s,
        Err(e) => panic!("Failed to get screen surface: {}", e.to_string()),
    };

    let key_press_surfaces = [
        &load_image("res/press.bmp"),
        &load_image("res/up.bmp"),
        &load_image("res/down.bmp"),
        &load_image("res/left.bmp"),
        &load_image("res/right.bmp"),
    ];

    let mut current_surface = key_press_surfaces[0];

    let mut event_pump = context.event_pump();
    let mut event_iter = event_pump.poll_iter();
    loop {
        let evt;
        match event_iter.next() {
            Some(x) => evt = x,
            None => continue
        }
        match evt {
            Event::Quit{..} => break,
            Event::KeyDown{ keycode: k, ..} => match k {
                KeyCode::Up    => current_surface = key_press_surfaces[1],
                KeyCode::Down  => current_surface = key_press_surfaces[2],
                KeyCode::Left  => current_surface = key_press_surfaces[3],
                KeyCode::Right => current_surface = key_press_surfaces[4],
                _              => current_surface = key_press_surfaces[0],
            },
            _ => {},
        }
        screen.blit(current_surface, None, None);
        window.update_surface();
    }

}
