mod constants;
mod direction;
mod draw_field;
mod field;
mod orientation;
mod piece;
mod piece_offsets;
mod piece_position;
mod piece_type;

extern crate sdl2;

use field::Field;
use piece::Piece;
use piece_position::PiecePosition;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::sys::SDL_GetTicks;
use sdl2::TimerSubsystem;
use std::cmp::min;
use std::time::Duration;

use crate::constants::{FIELD_HEIGHT, FIELD_WIDTH};
use crate::draw_field::clear_canvas;
use crate::draw_field::draw_field;
use crate::draw_field::draw_inside_field;

pub fn main() {
    let piece_type = rand::random();

    let mut field = Field::new(Piece::new(
        piece_type,
        PiecePosition::new(piece_type),
        orientation::Orientation::O,
    ));
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Blazing Tetris", 400, 800)
        .resizable()
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        clear_canvas(&mut canvas);
        let (x, y) = canvas.window().size();
        // defining the position of the drawing start
        let begin_x = (x - min(FIELD_WIDTH, x)) / 2;
        let begin_y = (y - min(FIELD_HEIGHT, y)) / 2;

        canvas.set_draw_color(Color::RGB(255, 255, 255));

        // Drawing the field centered
        draw_field(&mut canvas, &begin_x, &begin_y);

        // loop to draw cases
        draw_inside_field(&mut canvas, &mut field.get_data(), &begin_x, &begin_y);
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    field.move_piece(direction::Direction::Right);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => field.move_piece(direction::Direction::Left),
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => field.move_piece(direction::Direction::Down),
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => field.move_piece(direction::Direction::RotPlus),
                Event::KeyDown {
                    keycode: Some(Keycode::E),
                    ..
                } => field.move_piece(direction::Direction::RotMinus),
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
