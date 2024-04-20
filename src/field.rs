use crate::direction::Direction;
use crate::piece::Piece;

const COLUMNS: usize = 10;
const LINES: usize = 20;

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

    use crate::{orientation::Orientation, piece_position::PiecePosition, piece_type::PieceType};

    use super::*;
    #[test]
    fn test_init() {
        let mut data = [[false; COLUMNS]; LINES];
        let piece_type: PieceType = rand::random();
        let piece = Piece::new(piece_type, PiecePosition::new(piece_type), Orientation::O);
        for (x, y) in piece.get_position().pos {
            data[x as usize][y as usize] = true;
        }
        assert_eq!(Field::new(piece, [[false; COLUMNS]; LINES]).data, data)
    }

    #[test]
    fn test_move_piece_side() {
        let mut data = [[false; COLUMNS]; LINES];
        let piece_type: PieceType = rand::random();
        let piece: Piece = Piece::new(piece_type, PiecePosition::new(piece_type), Orientation::O);
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
        let piece = Piece::new(piece_type, PiecePosition::new(piece_type), Orientation::O);
        let field = Field::new(piece, [[false; COLUMNS]; LINES]);
        for (x, y) in piece.get_position().pos {
            data[(x + 1) as usize][y as usize] = true;
        }
        assert_eq!(field.drop_down_piece().data, data)
    }
}
