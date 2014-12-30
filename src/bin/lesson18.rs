extern crate sdl2;

use sdl2::event::{Event, poll_event};
use sdl2::keyboard::{get_keyboard_state};
use sdl2::video::{Window,WindowPos,OPENGL};
use sdl2::scancode::ScanCode;
use sdl2::surface::Surface;

fn load_image(filename: &str) -> Surface {
    match Surface::from_bmp(&Path::new(filename)) {
        Ok(s) => return s,
        Err(e) => panic!("Failed to load image surface: {}", e.to_string()),
    }
}

fn main() {
    sdl2::init(sdl2::INIT_EVERYTHING);

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

    'event: loop {
        let mut current_surface = key_press_surfaces[0];

        match poll_event() {
            Event::Quit(_) => break 'event,
            _ => {},
        }

        let kb_state = get_keyboard_state();

        if kb_state[ScanCode::Up] {
            current_surface = key_press_surfaces[1]
        } else if kb_state[ScanCode::Down] {
            current_surface = key_press_surfaces[2]
        } else if kb_state[ScanCode::Left] {
            current_surface = key_press_surfaces[3]
        } else if kb_state[ScanCode::Right] {
            current_surface = key_press_surfaces[4]
        }

        screen.blit(current_surface, None, None);
        window.update_surface();
    }

    sdl2::quit();
}
