extern crate sdl2;

use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{ACCELERATED, Renderer, RenderDriverIndex};
use sdl2::video::{Window,WindowPos,OPENGL};


fn main() {
    let context = sdl2::init(sdl2::INIT_EVERYTHING).unwrap();

    let window = match Window::new("lesson 8", WindowPos::PosCentered,
                                   WindowPos::PosCentered, 640, 480, OPENGL) {
        Ok(w)  => w,
        Err(e) => panic!("Failed to create window: {}", e.to_string()),
    };

    let renderer = match Renderer::from_window(
            window, RenderDriverIndex::Auto, ACCELERATED) {
        Ok(r)  => r,
        Err(e) => panic!("Failed to create renderer: {}", e.to_string()),
    };

    let mut event_pump = context.event_pump();
    'event: loop {
        for evt in event_pump.poll_iter() {
            match evt {
                Event::Quit{..} => break 'event,
                _ => {},
            }

            renderer.drawer().set_draw_color(Color::RGBA(255, 255, 255, 255));
            renderer.drawer().clear();

            let fill_rect = Rect::new(640 / 4, 480 / 4, 640 / 2, 480 / 2);
            renderer.drawer().set_draw_color(Color::RGBA(255, 0, 0, 255));
            renderer.drawer().fill_rect(fill_rect);

            let outline_rect = Rect::new(
                640 / 6, 480 / 6, 640 * 2 / 3, 480 * 2 / 3);
            renderer.drawer().set_draw_color(Color::RGBA(0, 255, 0, 255));
            renderer.drawer().draw_rect(outline_rect);

            renderer.drawer().set_draw_color(Color::RGBA(0, 0, 255, 255));
            renderer.drawer().draw_line(Point::new(0, 480 / 2),
                               Point::new(640, 480 / 2));

            renderer.drawer().set_draw_color(Color::RGBA(255, 255, 0, 255));
            for i in (0.. 480).step_by(4) {
                renderer.drawer().draw_point(Point::new(640 / 2, i));
            }

            renderer.drawer().present();
        }
    }

}
