extern crate sdl2;
extern crate sdl2_image;

use sdl2::pixels::Color;
use sdl2::event::{Event, poll_event};
use sdl2::render::{ACCELERATED, Renderer, RenderDriverIndex, Texture};
use sdl2::video::{Window,WindowPos,OPENGL};
use sdl2_image::LoadSurface;

fn load_image(filename: &str, renderer: &Renderer) -> Texture {
    let surface = match LoadSurface::from_file(&Path::new(filename)) {
        Ok(s) => s,
        Err(e) => panic!("Failed to load image surface: {}", e.to_string()),
    };
    let texture = renderer.create_texture_from_surface(&surface);
    texture.unwrap()
}

fn main() {
    sdl2::init(sdl2::INIT_EVERYTHING);
    sdl2_image::init(sdl2_image::INIT_PNG);

    let window = match Window::new("lesson 6", WindowPos::PosCentered,
                                   WindowPos::PosCentered, 640, 480, OPENGL) {
        Ok(w)  => w,
        Err(e) => panic!("Failed to create window: {}", e.to_string()),
    };

    let renderer = match Renderer::from_window(
            window, RenderDriverIndex::Auto, ACCELERATED) {
        Ok(r)  => r,
        Err(e) => panic!("Failed to create renderer: {}", e.to_string()),
    };

    renderer.set_draw_color(Color::RGBA(255, 255, 255, 255)).unwrap();

    let texture = load_image("res/texture.png", &renderer);
    'event: loop {
        match poll_event() {
            Event::Quit(_) => break 'event,
            _ => {},
        }

        renderer.clear().unwrap();
        renderer.copy(&texture, None, None).unwrap();
        renderer.present();
    }

    sdl2::quit();
}
