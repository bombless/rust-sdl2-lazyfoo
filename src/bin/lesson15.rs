extern crate sdl2;
extern crate sdl2_image;

use sdl2::event::{Event, poll_event};
use sdl2::keycode::KeyCode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{ACCELERATED, Renderer, RendererFlip, RenderDriverIndex,
                   Texture};
use sdl2::surface::Surface;
use sdl2::video::{Window,WindowPos,OPENGL};

use sdl2_image::LoadSurface;


pub struct Sprite {
    texture: Texture,
    width: i32,
    height: i32,
}

impl Sprite {
    pub fn new(filename: &Path, renderer: &Renderer) -> Sprite {
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
              clip: Option<Rect>, angle: f64, centre: Option<Point>,
              flip: Option<RendererFlip>) {
        let mut render_quad = Rect::new(x, y, self.width, self.height);
        match clip {
            Some(rect) => {
                render_quad.w = rect.w;
                render_quad.h = rect.h;
            }
            None => {}
        }

        let flip_mode = match flip {
            Some(f) => f,
            None => RendererFlip::None,
        };

        renderer.copy_ex(&self.texture, clip, Some(render_quad), angle,
                         centre, flip_mode).unwrap();
    }
}

fn main() {
    sdl2::init(sdl2::INIT_EVERYTHING);
    sdl2_image::init(sdl2_image::INIT_PNG);

    let window = match Window::new("lesson 15", WindowPos::PosCentered,
                                   WindowPos::PosCentered, 640, 480, OPENGL) {
        Ok(w)  => w,
        Err(e) => panic!("Failed to create window: {}", e.to_string()),
    };

    let renderer = match Renderer::from_window(
            window, RenderDriverIndex::Auto, ACCELERATED) {
        Ok(r)  => r,
        Err(e) => panic!("Failed to create renderer: {}", e.to_string()),
    };

    let arrow_texture = Sprite::new(&Path::new("res/arrow.png"), &renderer);

    let mut angle: f64 = 0.0;
    let mut flip_type = RendererFlip::None;
    'event: loop {
        match poll_event() {
            Event::Quit(_) => break 'event,
            Event::KeyDown(_,_,k,_,_,_) => match k {
                KeyCode::A => angle -= 60.0,
                KeyCode::D => angle += 60.0,
                KeyCode::Q => flip_type = RendererFlip::Horizontal,
                KeyCode::W => flip_type = RendererFlip::None,
                KeyCode::E => flip_type = RendererFlip::Vertical,
                _          => {},
            },
            _ => {},
        }

        renderer.set_draw_color(Color::RGBA(255, 255, 255, 255)).unwrap();
        renderer.clear().unwrap();

        arrow_texture.render(&renderer, (640 - arrow_texture.width) / 2,
                             (480 - arrow_texture.height) / 2, None,
                             angle, None, Some(flip_type));

        renderer.present();
    }

    sdl2::quit();
}
