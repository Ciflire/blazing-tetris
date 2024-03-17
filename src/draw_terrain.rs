const SQUARE_SIZE: u32 = 50;

use sdl2::{rect::Rect, render::WindowCanvas};

fn draw_rectangle_from_top_left(canvas: &mut WindowCanvas, x: i32, y: i32) {
    canvas.draw_rect(Rect::new(x, y, SQUARE_SIZE, SQUARE_SIZE));
}
