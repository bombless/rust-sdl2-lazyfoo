extern crate sdl2;
extern crate sdl2_image;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::render::{ACCELERATED, Renderer, RenderDriverIndex, Texture};
use sdl2::video::{Window,WindowPos,OPENGL};
use sdl2_image::LoadSurface;

use std::path::Path;

fn load_image<'a>(filename: &str, renderer: &'a Renderer) -> Texture<'a> {
    let surface = match LoadSurface::from_file(&Path::new(filename)) {
        Ok(s) => s,
        Err(e) => panic!("Failed to load image surface: {}", e.to_string()),
    };
    let texture = renderer.create_texture_from_surface(&surface);
    texture.unwrap()
}

fn main() {
    let context = sdl2::init(sdl2::INIT_EVERYTHING).unwrap();
    sdl2_image::init(sdl2_image::INIT_PNG);

    let window = match Window::new("lesson 7", WindowPos::PosCentered,
                                   WindowPos::PosCentered, 640, 480, OPENGL) {
        Ok(w)  => w,
        Err(e) => panic!("Failed to create window: {}", e.to_string()),
    };

    let renderer = match Renderer::from_window(
            window, RenderDriverIndex::Auto, ACCELERATED) {
        Ok(r)  => r,
        Err(e) => panic!("Failed to create renderer: {}", e.to_string()),
    };

    renderer.drawer().set_draw_color(Color::RGBA(255, 255, 255, 255));

    let texture = load_image("res/texture.png", &renderer);

    let mut event_pump = context.event_pump();
    'event: loop {
        for evt in event_pump.poll_iter() {
            match evt {
                Event::Quit{..} => break 'event,
                _ => {},
            }

            renderer.drawer().clear();
            renderer.drawer().copy(&texture, None, None);
            renderer.drawer().present();
        }
    }
}
