extern crate sdl2;
extern crate sdl2_image;

use sdl2::event::Event;
use sdl2::keycode::KeyCode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{ACCELERATED, BlendMode, Renderer, RenderDriverIndex,
                   Texture};
use sdl2::surface::Surface;
use sdl2::video::{Window,WindowPos,OPENGL};

use sdl2_image::LoadSurface;

use std::path::Path;

pub struct Sprite<'a> {
    texture: Texture<'a>,
    width: i32,
    height: i32,
}

impl<'a> Sprite<'a> {
    pub fn new(filename: &Path, renderer: &'a Renderer) -> Sprite<'a> {
        let surface: Surface = match LoadSurface::from_file(filename) {
            Ok(s) => s,
            Err(e) => panic!("Failed to load image surface: {}", e.to_string()),
        };

        surface.set_color_key(true, Color::RGB(0, 255, 255)).unwrap();

        let width = surface.get_width();
        let height = surface.get_height();

        let texture = renderer.create_texture_from_surface(&surface).unwrap();

        Sprite {
            texture: texture,
            width: width as i32,
            height: height as i32,
        }
    }

    fn set_alpha(&mut self, alpha: u8) {
        self.texture.set_alpha_mod(alpha)
    }

    fn set_blend_mode(&mut self, blend_mode: BlendMode) {
        self.texture.set_blend_mode(blend_mode)
    }

    fn render(&self, renderer: &Renderer, x: i32, y: i32,
              clip: Option<Rect>) {
        let mut render_quad = Rect::new(x, y, self.width, self.height);

        match clip {
            Some(rect) => {
                render_quad.w = rect.w;
                render_quad.h = rect.h;
            }
            None => {}
        }

        renderer.drawer().copy(&self.texture, clip, Some(render_quad))
    }
}

fn main() {
    let context = sdl2::init(sdl2::INIT_EVERYTHING).unwrap();
    sdl2_image::init(sdl2_image::INIT_PNG);

    let window = match Window::new("lesson 13", WindowPos::PosCentered,
                                   WindowPos::PosCentered, 640, 480, OPENGL) {
        Ok(w)  => w,
        Err(e) => panic!("Failed to create window: {}", e.to_string()),
    };

    let renderer = match Renderer::from_window(
            window, RenderDriverIndex::Auto, ACCELERATED) {
        Ok(r)  => r,
        Err(e) => panic!("Failed to create renderer: {}", e.to_string()),
    };

    let mut modulated_texture = Sprite::new(
            &Path::new("res/fadeout.png"), &renderer);
    modulated_texture.set_blend_mode(BlendMode::Blend);

    let background_texture = Sprite::new(
            &Path::new("res/fadein.png"), &renderer);

    // Use i16 because a u8 will automatically wrap around, which isn't quite
    // what we want.
    let mut a: i16 = 255;

    let mut event_pump = context.event_pump();
    'event: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..} => break 'event,
                Event::KeyDown{ keycode: k, ..} => match k {
                    KeyCode::W => a = if a + 32 > 255 { 255 } else { a + 32 },
                    KeyCode::S => a = if a - 32 < 0 { 0 } else { a - 32 },
                    _          => {},
                },
                _ => {},
            }

            renderer.drawer().set_draw_color(Color::RGBA(255, 255, 255, 255));
            renderer.drawer().clear();

            background_texture.render(&renderer, 0, 0, None);
            modulated_texture.set_alpha(a as u8);
            modulated_texture.render(&renderer, 0, 0, None);

            renderer.drawer().present();
        }
    }

}
