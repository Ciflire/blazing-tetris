extern crate sdl2;

mod constants;
mod description;
mod draw_field;

use constants::{FIELD_HEIGHT, FIELD_WIDTH};
use description::instantialize_field;
use draw_field::{clear_canvas, draw_field, draw_inside_field};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use sdl2::video::Window;
use sdl2::{Sdl, VideoSubsystem};
use std::cmp::{max, min};
use std::time::Duration;

pub fn main() {
    let mut field = instantialize_field();
    let sdl_context: Sdl = sdl2::init().unwrap();
    let video_subsystem: VideoSubsystem = sdl_context.video().unwrap();

    let window: Window = video_subsystem
        .window("Blazing Tetris", 800, 700)
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
        // drawing main field

        // defining the position of the drawing start
        let begin_x = (x - min(FIELD_WIDTH, x)) / 2;
        let begin_y = (y - min(FIELD_HEIGHT, y)) / 2;

        println!("{} {} {} {}", x, y, begin_x, begin_y);

        // Drawing the field centered
        draw_field(&mut canvas, &begin_x, &begin_y);

        // loop to draw cases
        draw_inside_field(&mut canvas, &mut field, &begin_x, &begin_y);

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
