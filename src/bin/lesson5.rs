extern crate sdl2;

use sdl2::event::{Event, poll_event};
use sdl2::pixels::PixelFormat;
use sdl2::rect::Rect;
use sdl2::surface::Surface;
use sdl2::video::{Window,WindowPos,OPENGL};

fn load_image(filename: &str, format: PixelFormat) -> Surface {
    let image_surface = match Surface::from_bmp(&Path::new(filename)) {
        Ok(s) => s,
        Err(e) => panic!("Failed to load image surface: {}", e.to_string()),
    };
    let image_surface = image_surface.convert(&format);
    image_surface.unwrap()
}

fn main() {
    sdl2::init(sdl2::INIT_EVERYTHING);

    let window = match Window::new("lesson 5", WindowPos::PosCentered,
                                   WindowPos::PosCentered, 640, 480, OPENGL) {
        Ok(w)  => w,
        Err(e) => panic!("Failed to create window: {}", e.to_string()),
    };

    let mut screen = match window.get_surface() {
        Ok(s)  => s,
        Err(e) => panic!("Failed to get screen surface: {}", e.to_string()),
    };

    let surface = load_image("res/stretch.bmp", screen.get_pixel_format());
    'event: loop {
        match poll_event() {
            Event::Quit(_) => break 'event,
            _ => {},
        }

        // rust-sdl2 isn't using the newest interface, hence softstretch.
        match surface.soft_stretch(None, &mut screen,
                                   Some(Rect::new(0, 0, 640, 480))) {
            Ok(_) => {},
            Err(e) => panic!("error: {}", e.to_string()),
        }
        window.update_surface();
    }

    sdl2::quit();
}
