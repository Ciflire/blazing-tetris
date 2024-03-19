extern crate sdl2;

mod draw_terrain;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::video::Window;
use sdl2::{Sdl, VideoSubsystem};
use std::cmp::{max, min};
use std::time::Duration;

const TERRAIN_WIDTH: u32 = 515;
const TERRAIN_HEIGHT: u32 = 765;

pub fn main() {
    let sdl_context: Sdl = sdl2::init().unwrap();
    let video_subsystem: VideoSubsystem = sdl_context.video().unwrap();

    let window: Window = video_subsystem
        .window("rust-sdl2 demo", 800, 600)
        .resizable()
        .position_centered()
        .build()
        .unwrap();

    let mut canvas: WindowCanvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    // Set background color to black
    canvas.set_draw_color((255, 255, 255));

    'running: loop {
        // gets the canvas size
        let (x, y) = canvas.window().size();
        //clear the canvas before redrawing
        clear_canvas(&mut canvas);
        match canvas.draw_rect(Rect::new(
            std::cmp::max((x - min(TERRAIN_WIDTH, x)) / 2, 0) as i32,
            std::cmp::max((y - min(TERRAIN_HEIGHT, y)) / 2, 0) as i32,
            TERRAIN_WIDTH,
            TERRAIN_HEIGHT,
        )) {
            Ok(f) => f,
            Err(_) => panic!("Error drawing rectangle"),
        };
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn clear_canvas(canvas: &mut WindowCanvas) {
    let previous_color = canvas.draw_color();
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    canvas.set_draw_color(previous_color);
}
