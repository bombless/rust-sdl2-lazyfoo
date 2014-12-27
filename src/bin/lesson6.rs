extern crate sdl2;
extern crate sdl2_image;

use sdl2::event::{Event, poll_event};
use sdl2::pixels::PixelFormat;
use sdl2::surface::Surface;
use sdl2::video::{Window,WindowPos,OPENGL};
use sdl2_image::LoadSurface;

fn load_image(filename: &str, format: PixelFormat) -> Surface {
    let image_surface = match LoadSurface::from_file(&Path::new(filename)) {
        Ok(s) => s,
        Err(e) => panic!("Failed to load image surface: {}", e.to_string()),
    };
    let image_surface = image_surface.convert(&format);
    image_surface.unwrap()
}

fn main() {
    sdl2::init(sdl2::INIT_EVERYTHING);
    sdl2_image::init(sdl2_image::INIT_PNG);

    let window = match Window::new("lesson 6", WindowPos::PosCentered,
                                   WindowPos::PosCentered, 640, 480, OPENGL) {
        Ok(w)  => w,
        Err(e) => panic!("Failed to create window: {}", e.to_string()),
    };

    let screen = match window.get_surface() {
        Ok(s)  => s,
        Err(e) => panic!("Failed to get screen surface: {}", e.to_string()),
    };

    let surface = load_image("res/loaded.png", screen.get_pixel_format());
    'event: loop {
        match poll_event() {
            Event::Quit(_) => break 'event,
            _ => {},
        }

        screen.blit(&surface, None, None);
        window.update_surface();
    }

    sdl2::quit();
}
