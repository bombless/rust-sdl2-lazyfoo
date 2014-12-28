extern crate sdl2;
extern crate sdl2_image;

use sdl2::event::{Event, poll_event};
use sdl2::keycode::KeyCode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{ACCELERATED, BlendMode, Renderer, RenderDriverIndex,
                   Texture};
use sdl2::video::{Window,WindowPos,OPENGL};

use sdl2_image::LoadSurface;


pub struct Sprite {
    texture: Texture,
    width: i32,
    height: i32,
}

impl Sprite {
    pub fn new(filename: &Path, renderer: &Renderer) -> Sprite {
        let surface = match LoadSurface::from_file(filename) {
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

    fn set_alpha(&self, alpha: u8) {
        self.texture.set_alpha_mod(alpha).unwrap();
    }

    fn set_blend_mode(&self, blend_mode: BlendMode) {
        self.texture.set_blend_mode(blend_mode).unwrap();
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

        renderer.copy(&self.texture, clip, Some(render_quad)).unwrap();
    }
}

fn main() {
    sdl2::init(sdl2::INIT_EVERYTHING);
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

    let modulated_texture = Sprite::new(
            &Path::new("res/fadeout.png"), &renderer);
    modulated_texture.set_blend_mode(BlendMode::Blend);

    let background_texture = Sprite::new(
            &Path::new("res/fadein.png"), &renderer);

    // Use i16 because a u8 will automatically wrap around, which isn't quite
    // what we want.
    let mut a: i16 = 255;
    'event: loop {
        match poll_event() {
            Event::Quit(_) => break 'event,
            Event::KeyDown(_,_,k,_,_,_) => match k {
                KeyCode::W => a = if a + 32 > 255 { 255 } else { a + 32 },
                KeyCode::S => a = if a - 32 < 0 { 0 } else { a - 32 },
                _          => {},
            },
            _ => {},
        }

        renderer.set_draw_color(Color::RGBA(255, 255, 255, 255)).unwrap();
        renderer.clear().unwrap();

        background_texture.render(&renderer, 0, 0, None);
        modulated_texture.set_alpha(a as u8);
        modulated_texture.render(&renderer, 0, 0, None);

        renderer.present();
    }

    sdl2::quit();
}