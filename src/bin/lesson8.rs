extern crate sdl2;
extern crate sdl2_image;

use sdl2::event::{Event, poll_event};
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{ACCELERATED, Renderer, RenderDriverIndex};
use sdl2::video::{Window,WindowPos,OPENGL};

use std::iter::range_step;

fn main() {
    sdl2::init(sdl2::INIT_EVERYTHING);

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

    'event: loop {
        match poll_event() {
            Event::Quit(_) => break 'event,
            _ => {},
        }

        renderer.set_draw_color(Color::RGBA(255, 255, 255, 255)).unwrap();
        renderer.clear().unwrap();

        let fill_rect = Rect::new(640 / 4, 480 / 4, 640 / 2, 480 / 2);
        renderer.set_draw_color(Color::RGBA(255, 0, 0, 255)).unwrap();
        renderer.fill_rect(&fill_rect).unwrap();

        let outline_rect = Rect::new(
            640 / 6, 480 / 6, 640 * 2 / 3, 480 * 2 / 3);
        renderer.set_draw_color(Color::RGBA(0, 255, 0, 255)).unwrap();
        renderer.draw_rect(&outline_rect).unwrap();

        renderer.set_draw_color(Color::RGBA(0, 0, 255, 255)).unwrap();
        renderer.draw_line(Point::new(0, 480 / 2),
                           Point::new(640, 480 / 2)).unwrap();

        renderer.set_draw_color(Color::RGBA(255, 255, 0, 255)).unwrap();
        for i in range_step(0, 480, 4) {
            renderer.draw_point(Point::new(640 / 2, i)).unwrap();
        }

        renderer.present();
    }

    sdl2::quit();
}
