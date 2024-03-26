use crate::constants::{FIELD_HEIGHT, FIELD_WIDTH, SQUARE_SIZE};

use sdl2::{pixels::Color, rect::Rect, render::WindowCanvas};

/// This function draws the main rectangle where cases will be
///Always draw the rectangle CENTERED

pub fn draw_field(canvas: &mut WindowCanvas, x: &u32, y: &u32) {
    let format: Rect = Rect::new(*x as i32, *y as i32, FIELD_WIDTH, FIELD_HEIGHT);
    canvas.draw_rect(format).unwrap();
}

fn draw_case(canvas: &mut WindowCanvas, x: &u32, y: &u32) {
    let format: Rect = Rect::new(*x as i32, *y as i32, SQUARE_SIZE, SQUARE_SIZE);
    canvas.draw_rect(format).unwrap();
}

fn draw_case_filled(canvas: &mut WindowCanvas, x: &u32, y: &u32) {
    let format: Rect = Rect::new(*x as i32, *y as i32, SQUARE_SIZE, SQUARE_SIZE);
    canvas.fill_rect(format).unwrap();
}

/// Draws the inside field
pub fn draw_inside_field(
    canvas: &mut WindowCanvas,
    field: &mut [[bool; 10]; 21],
    begin_x: &u32,
    begin_y: &u32,
) {
    for i in 0..20 {
        for j in 0..10 {
            let x_pos: u32 = *begin_x + j * (SQUARE_SIZE);
            let y_pos: u32 = *begin_y + i * (SQUARE_SIZE);
            match field[(i + 1) as usize][j as usize] {
                true => draw_case_filled(canvas, &x_pos, &y_pos),
                false => (),
            };
        }
    }
}

pub fn clear_canvas(canvas: &mut WindowCanvas) {
    let previous_color = canvas.draw_color();
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    canvas.set_draw_color(previous_color);
}
