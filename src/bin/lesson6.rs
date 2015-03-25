extern crate sdl2;
extern crate sdl2_image;

use sdl2::event::Event;
use sdl2::pixels::PixelFormat;
use sdl2::surface::Surface;
use sdl2::video::{Window,WindowPos,OPENGL};
use sdl2_image::LoadSurface;

use std::path::Path;

fn load_image(filename: &str, format: PixelFormat) -> Surface {
    let image_surface: Surface = match LoadSurface::from_file(&Path::new(filename)) {
        Ok(s) => s,
        Err(e) => panic!("Failed to load image surface: {}", e.to_string()),
    };
    let image_surface = image_surface.convert(&format);
    image_surface.unwrap()
}

fn main() {
    let context = sdl2::init(sdl2::INIT_EVERYTHING).unwrap();
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

    let mut event_pump = context.event_pump();
    'event: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..} => break 'event,
                _ => {},
            }

            screen.blit(&surface, None, None);
            window.update_surface();
        }
    }
}
