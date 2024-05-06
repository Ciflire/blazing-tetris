#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
    Down,
    RotPlus,  // Clockwise
    RotMinus, // Clockwise
}
