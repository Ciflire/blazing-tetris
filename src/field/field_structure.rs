const COLUMNS: usize = 10;
const LINE: usize = 20;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    O,
    R,
    Half,
    L,
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
    data: [[bool; COLUMNS]; LINE],
}

impl Field {
    pub fn new(current_piece: Piece, data: [[bool; COLUMNS]; LINE]) -> Self {
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

    pub fn side_move_piece(mut self, direction: &str) -> Self {
        let is_not_left: bool = {
            for (x, y) in self.current_piece.get_position().pos {
                if y == 0 {
                    false;
                } else if self.data[x as usize][(y - 1) as usize] {
                    false;
                }
            }
            true
        };
        let is_not_right: bool = {
            for (x, y) in self.current_piece.get_position().pos {
                if y < 9 {
                    false;
                } else if self.data[x as usize][(y + 1) as usize] {
                    false;
                }
            }
            true
        };
        if direction == "Left" {
            if is_not_left {
                let mut i = 0;
                for (x, y) in self.current_piece.get_position().pos {
                    self.data[x as usize][y as usize] = false;
                    self.current_piece.get_position().pos[i] = (x, y - 1);
                    self.data[x as usize][(y - 1) as usize] = true;
                    i += 1;
                }
            } else {
                println!("Reached border")
            }
        }
        if direction == "Right" {
            if is_not_right {
                let mut i = 0;
                for (x, y) in self.current_piece.get_position().pos {
                    self.data[x as usize][y as usize] = false;
                    self.current_piece.get_position().pos[i] = (x, y + 1);
                    self.data[x as usize][(y + 1) as usize] = true;
                    i += 1;
                }
            } else {
                println!("Reached border")
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
                self.current_piece.get_position().pos[i] = (x + 1, y);
                self.data[(x + 1) as usize][y as usize] = true;
                i += 1;
            }
        }
        self
    }
}

#[cfg(test)]
mod test {
    use std::path::Component;

    use super::*;
    use rand::Rng;
    #[test]
    fn test_init() {
        let mut data = [[false; COLUMNS]; LINE];
        let piece = Piece {
            piece_type: PieceType::I,
            position: PiecePosition::new(PieceType::I),
            orientation: Orientation::O,
        };
        data[0][0] = true;
        data[0][1] = true;
        data[0][2] = true;
        data[0][3] = true;
        assert_eq!(Field::new(piece, [[false; COLUMNS]; LINE]).data, data)
    }

    #[test]
    fn test_move_piece_side() {}
}
