extern crate sdl2;
extern crate sdl2_image;

use sdl2::event::{Event, poll_event};
use sdl2::mouse::get_mouse_state;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{ACCELERATED, Renderer, RendererFlip, RenderDriverIndex,
                   Texture};
use sdl2::video::{Window,WindowPos,OPENGL};

use sdl2_image::LoadSurface;

use std::rc::Rc;


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

enum ButtonSpriteType {
    MouseOut,
    MouseOverMotion,
    MouseDown,
    MouseUp,
}

struct Button {
    position: Point,
    btn_type: ButtonSpriteType,
    height: i32,
    width: i32,
    sprite_sheet: Rc<Sprite>,
}

impl Button {
    pub fn new(x: i32, y: i32, width: i32, height: i32,
               sprite_sheet: Rc<Sprite>) -> Button {
        Button {
            position:  Point::new(x, y),
            height: height,
            width: width,
            btn_type: ButtonSpriteType::MouseOut,
            sprite_sheet: sprite_sheet,
        }
    }

    pub fn handle_event(&mut self, e: &Event) {
        let (_, x, y) = get_mouse_state();

        if self.mouse_is_in_button(x as i32, y as i32) {
            match e {
                &Event::MouseMotion(..) => {
                    self.btn_type = ButtonSpriteType::MouseOverMotion
                }
                &Event::MouseButtonDown(..) => {
                    self.btn_type = ButtonSpriteType::MouseDown
                }
                &Event::MouseButtonUp(..) => {
                    self.btn_type = ButtonSpriteType::MouseUp
                }
                _ => return,
            }
        } else {
            self.btn_type = ButtonSpriteType::MouseOut;
        }
    }

    pub fn render(&self, renderer: &Renderer) {
        let clip = match self.btn_type {
            ButtonSpriteType::MouseOut => Rect::new(0, 0, 300, 200),
            ButtonSpriteType::MouseOverMotion => Rect::new(0, 200, 300, 200),
            ButtonSpriteType::MouseDown => Rect::new(0, 400, 300, 200),
            ButtonSpriteType::MouseUp => Rect::new(0, 600, 300, 200),

        };
        self.sprite_sheet.render(renderer, self.position.x, self.position.y,
                                 Some(clip), 0.0, None, None);
    }

    fn mouse_is_in_button(&self, mouse_x: i32, mouse_y: i32) -> bool {
        if mouse_x < self.position.x {
            return false;
        } else if mouse_x > self.position.x + self.width {
            return false;
        } else if mouse_y < self.position.y {
            return false;
        } else if mouse_y > self.position.y + self.height {
            return false;
        }

        true
    }
}

fn main() {
    sdl2::init(sdl2::INIT_EVERYTHING);
    sdl2_image::init(sdl2_image::INIT_PNG);

    let window = match Window::new("lesson 17", WindowPos::PosCentered,
                                   WindowPos::PosCentered, 640, 480, OPENGL) {
        Ok(w)  => w,
        Err(e) => panic!("Failed to create window: {}", e.to_string()),
    };

    let renderer = match Renderer::from_window(
            window, RenderDriverIndex::Auto, ACCELERATED) {
        Ok(r)  => r,
        Err(e) => panic!("Failed to create renderer: {}", e.to_string()),
    };

    let button_sprite_sheet = Rc::new(Sprite::new(&Path::new("res/button.png"),
                                                  &renderer));
    let mut button1 = Button::new(0, 0, 300, 200,
                                  button_sprite_sheet.clone());
    let mut button2 = Button::new(640 - 300, 0, 300, 200,
                                  button_sprite_sheet.clone());
    let mut button3 = Button::new(0, 480 - 200, 300, 200,
                                  button_sprite_sheet.clone());
    let mut button4 = Button::new(640 - 300, 480 - 200, 300, 200,
                                  button_sprite_sheet.clone());

    'event: loop {
        match poll_event() {
            Event::Quit(_) => break 'event,
            e => {
                button1.handle_event(&e);
                button2.handle_event(&e);
                button3.handle_event(&e);
                button4.handle_event(&e);
            }
        }

        renderer.set_draw_color(Color::RGBA(255, 255, 255, 255)).unwrap();
        renderer.clear().unwrap();

        button1.render(&renderer);
        button2.render(&renderer);
        button3.render(&renderer);
        button4.render(&renderer);

        renderer.present();
    }

    sdl2::quit();
}
