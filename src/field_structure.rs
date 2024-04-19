const COLUMNS: usize = 10;
const LINES: usize = 20;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceType {
    I,
    O,
    T,
    J,
    L,
    S,
    Z,
}

impl Distribution<PieceType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PieceType {
        match rng.gen_range(0..7) {
            0 => PieceType::I,
            1 => PieceType::O,
            2 => PieceType::T,
            3 => PieceType::J,
            4 => PieceType::L,
            5 => PieceType::S,
            _ => PieceType::Z,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    O,
    R,
    Half,
    L,
}

impl Distribution<Orientation> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Orientation {
        match rng.gen_range(0..4) {
            0 => Orientation::O,
            1 => Orientation::R,
            2 => Orientation::Half,
            _ => Orientation::L,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PieceOffsets {
    offsets: [(u32, u32); 4],
}

#[derive(Debug, Clone, Copy)]
pub struct PiecePosition {
    pos: [(u32, u32); 4],
}

impl PiecePosition {
    fn new(piece_type: PieceType) -> Self {
        match piece_type {
            PieceType::I => PiecePosition {
                pos: [(0, 0), (0, 1), (0, 2), (0, 3)],
            },
            PieceType::O => PiecePosition {
                pos: [(0, 0), (0, 1), (1, 0), (1, 1)],
            },
            PieceType::T => PiecePosition {
                pos: [(0, 1), (1, 0), (1, 1), (1, 2)],
            },
            PieceType::J => PiecePosition {
                pos: [(0, 0), (1, 0), (1, 1), (1, 2)],
            },
            PieceType::L => PiecePosition {
                pos: [(1, 0), (1, 1), (1, 2), (0, 2)],
            },
            PieceType::S => PiecePosition {
                pos: [(0, 1), (0, 2), (1, 0), (1, 1)],
            },
            PieceType::Z => PiecePosition {
                pos: [(0, 0), (0, 1), (1, 1), (1, 2)],
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Piece {
    piece_type: PieceType,
    position: PiecePosition,
    orientation: Orientation,
}

impl Piece {
    pub fn new(piece_type: PieceType, position: PiecePosition, orientation: Orientation) -> Self {
        Piece {
            piece_type,
            position,
            orientation,
        }
    }

    fn get_piece_type(&self) -> PieceType {
        self.piece_type
    }

    fn get_position(&self) -> PiecePosition {
        self.position
    }

    fn get_orientation(&self) -> Orientation {
        self.orientation
    }

    fn move_piece_down(&self) -> Self {
        let mut position = self.get_position().pos;
        for i in 0..4 {
            position[i as usize].0 += 1;
        }
        *self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Field {
    current_piece: Piece,
    data: [[bool; COLUMNS]; LINES],
}

impl Field {
    pub fn new(current_piece: Piece, data: [[bool; COLUMNS]; LINES]) -> Self {
        Self {
            current_piece,
            data,
        }
        .place_piece(current_piece)
    }

    pub fn place_piece(mut self, new_piece: Piece) -> Self {
        for elt in new_piece.get_position().pos {
            self.data[elt.0 as usize][elt.1 as usize] = true;
        }
        self
    }

    pub fn side_move_piece(mut self, direction: Direction) -> Self {
        let will_move: bool = {
            for (x, y) in self.current_piece.get_position().pos {
                if (y == 0
                    && direction == Direction::Left
                    && self.data[x as usize][(y + 1) as usize])
                    || (y == 9
                        && direction == Direction::Right
                        && self.data[x as usize][(y + 1) as usize])
                {
                    false;
                }
            }
            true
        };
        if will_move {
            match direction {
                Direction::Left => {
                    let mut i = 0;
                    for (x, y) in self.current_piece.get_position().pos {
                        self.data[x as usize][y as usize] = false;
                        self.current_piece.position.pos[i] = (x, y - 1);
                        i += 1;
                    }
                    for (x, y) in self.current_piece.get_position().pos {
                        self.data[x as usize][y as usize] = true;
                    }
                }
                Direction::Right => {
                    let mut i = 0;
                    for (x, y) in self.current_piece.get_position().pos {
                        self.data[x as usize][y as usize] = false;
                        println!("{} {}", x, y);
                        self.current_piece.position.pos[i] = (x, y + 1);
                        println!("{:?}", self.current_piece.get_position().pos);
                        i += 1;
                    }
                    for (x, y) in self.current_piece.get_position().pos {
                        self.data[x as usize][y as usize] = true;
                    }
                }
            }
        }
        self
    }

    pub fn drop_down_piece(mut self) -> Self {
        let no_piece_down = {
            for (x, y) in self.current_piece.get_position().pos {
                if x == 20 {
                    false;
                } else if self.data[(x + 1) as usize][y as usize] {
                    false;
                }
            }
            true
        };
        let mut i = 0;
        if no_piece_down {
            for (x, y) in self.current_piece.get_position().pos {
                self.data[x as usize][y as usize] = false;
                self.current_piece.position.pos[i] = (x + 1, y);
                i += 1;
            }
            for (x, y) in self.current_piece.get_position().pos {
                self.data[x as usize][y as usize] = true;
            }
        }
        self
    }
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn test_init() {
        let mut data = [[false; COLUMNS]; LINES];
        let piece_type = rand::random();
        let piece = Piece {
            piece_type,
            position: PiecePosition::new(piece_type),
            orientation: Orientation::O,
        };
        for (x, y) in piece.get_position().pos {
            data[x as usize][y as usize] = true;
        }
        assert_eq!(Field::new(piece, [[false; COLUMNS]; LINES]).data, data)
    }

    #[test]
    fn test_move_piece_side() {
        let mut data = [[false; COLUMNS]; LINES];
        let piece_type: PieceType = rand::random();
        let piece = Piece {
            piece_type,
            position: PiecePosition::new(piece_type),
            orientation: Orientation::O,
        };
        let field = Field::new(piece, [[false; COLUMNS]; LINES]);
        for (x, y) in piece.get_position().pos {
            data[x as usize][(y + 1) as usize] = true;
        }
        assert_eq!(field.side_move_piece(Direction::Right).data, data)
    }

    #[test]
    fn test_drop_down_piece() {
        let mut data = [[false; COLUMNS]; LINES];
        let piece_type: PieceType = rand::random();
        let piece = Piece {
            piece_type,
            position: PiecePosition::new(piece_type),
            orientation: Orientation::O,
        };
        let field = Field::new(piece, [[false; COLUMNS]; LINES]);
        for (x, y) in piece.get_position().pos {
            data[(x + 1) as usize][y as usize] = true;
        }
        assert_eq!(field.drop_down_piece().data, data)
    }
}
