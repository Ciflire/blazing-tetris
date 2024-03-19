enum Colors {
    Black,
}

fn color_of(color: Color) -> (i32, i32, i32) {
    match color {
        Black => (0, 0, 0),
    }
}
