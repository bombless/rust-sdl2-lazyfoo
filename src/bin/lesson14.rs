extern crate sdl2;
extern crate sdl2_image;

use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{ACCELERATED, PRESENTVSYNC, Renderer, RenderDriverIndex,
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

    let window = match Window::new("lesson 14", WindowPos::PosCentered,
                                   WindowPos::PosCentered, 640, 480, OPENGL) {
        Ok(w)  => w,
        Err(e) => panic!("Failed to create window: {}", e.to_string()),
    };

    let renderer = match Renderer::from_window(
            window, RenderDriverIndex::Auto, ACCELERATED | PRESENTVSYNC) {
        Ok(r)  => r,
        Err(e) => panic!("Failed to create renderer: {}", e.to_string()),
    };

    let sprite_sheet = Sprite::new(&Path::new("res/foo2.png"), &renderer);
    let sprite_clips = [
        Rect::new(0, 0, 64, 205),
        Rect::new(64, 0, 64, 205),
        Rect::new(128, 0, 64, 205),
        Rect::new(196, 0, 64, 205),
    ];

    let mut frame = 0usize;

    let mut event_pump = context.event_pump();
    'event: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..} => break 'event,
                _ => {},
            }

            renderer.drawer().set_draw_color(Color::RGBA(255, 255, 255, 255));
            renderer.drawer().clear();

            let current_clip = sprite_clips[frame / 4];
            sprite_sheet.render(&renderer, (640 - current_clip.w) / 2,
                                (480 - current_clip.h) / 2, Some(current_clip));

            frame += 1;
            if frame / 4 >= 4 {
                frame = 0usize;
            }

            renderer.drawer().present();
        }
    }

}
