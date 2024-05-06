use std::iter::Map;

use crate::constants::{COLUMNS, LINES};
use crate::direction::Direction;
use crate::piece::Piece;

#[derive(Debug, Clone)]
pub struct Field {
    current_piece: Piece,
    old_pieces: Vec<Piece>,
    data: [[bool; COLUMNS]; LINES],
}

impl Field {
    pub fn new(current_piece: Piece) -> Self {
        let mut res = Self {
            current_piece,
            old_pieces: vec![],
            data: [[false; COLUMNS]; LINES],
        };
        res.place_piece(current_piece);
        res
    }

    pub fn get_data(&self) -> [[bool; COLUMNS]; LINES] {
        self.data
    }

    fn place_piece(&mut self, new_piece: Piece) {
        for elt in new_piece.get_position().pos {
            self.data[elt.0 as usize][elt.1 as usize] = true;
        }
    }

    fn remove_piece(&mut self) {
        for (x, y) in self.current_piece.get_position().pos {
            self.data[x as usize][y as usize] = false;
        }
    }

    fn no_piece_around(&self, direction: &Direction) -> bool {
        let piece_to_move = self.current_piece;
        let other_pieces = &self.old_pieces;

        let (x_min, y_min) = (
            piece_to_move
                .get_position()
                .pos
                .map(|t| t.0)
                .iter()
                .min()
                .unwrap()
                .to_owned() as i32,
            piece_to_move
                .get_position()
                .pos
                .map(|t| t.1)
                .iter()
                .min()
                .unwrap()
                .to_owned() as i32,
        );
        // test for walls
        for (x_current_piece, y_current_piece) in piece_to_move.get_position().pos {
            if (*direction == Direction::Left && y_current_piece == 0)
                || (*direction == Direction::Right && y_current_piece == 9)
                || (*direction == Direction::Down && x_current_piece == 19)
            {
                return false;
            }
        }
        match (
            *direction,
            piece_to_move.piece_type,
            piece_to_move.orientation,
        ) {
            (Direction::Left, _, _) => {
                for (x_current_piece, y_current_piece) in piece_to_move.get_position().pos {
                    for piece in other_pieces {
                        for (x_other_piece, y_other_piece) in piece.get_position().pos {
                            if x_current_piece + 1 == x_other_piece
                                && y_current_piece == y_other_piece
                            {
                                return false;
                            }
                        }
                    }
                }
                return true;
            }

            (Direction::Right, _, _) => {
                for (x_current_piece, y_current_piece) in piece_to_move.get_position().pos {
                    for piece in other_pieces {
                        for (x_other_piece, y_other_piece) in piece.get_position().pos {
                            if x_current_piece - 1 == x_other_piece
                                && y_current_piece == y_other_piece
                            {
                                return false;
                            }
                        }
                    }
                }
                return true;
            }
            (Direction::Down, _, _) => {
                for (x_current_piece, y_current_piece) in piece_to_move.get_position().pos {
                    for piece in other_pieces {
                        for (x_other_piece, y_other_piece) in piece.get_position().pos {
                            if x_current_piece == x_other_piece
                                && y_current_piece + 1 == y_other_piece
                            {
                                return false;
                            }
                        }
                    }
                }
                true
            }
            (
                Direction::RotPlus,
                crate::piece_type::PieceType::I,
                crate::orientation::Orientation::O,
            ) => {
                let pos_after_rotation = [
                    (x_min - 1, y_min + 2),
                    (x_min, y_min + 2),
                    (x_min + 1, y_min + 2),
                    (x_min + 2, y_min + 2),
                ];

                let check_validity_min = {
                    for (i, j) in pos_after_rotation {
                        if i < 0 || j > 19 {
                            return false;
                        }
                    }
                    true
                };

                check_validity_min
            }
            (
                Direction::RotPlus,
                crate::piece_type::PieceType::I,
                crate::orientation::Orientation::R,
            ) => {
                let pos_after_rotation = [
                    (x_min + 2, y_min - 2),
                    (x_min + 2, y_min - 1),
                    (x_min + 2, y_min),
                    (x_min + 2, y_min + 1),
                ];

                let check_validity = {
                    for (i, j) in pos_after_rotation {
                        if i < 0 || j > 19 {
                            return false;
                        }
                    }
                    true
                };

                check_validity
            }
            (
                Direction::RotPlus,
                crate::piece_type::PieceType::I,
                crate::orientation::Orientation::Half,
            ) => {
                let pos_after_rotation = [
                    (x_min - 2, y_min + 1),
                    (x_min - 1, y_min + 1),
                    (x_min, y_min + 1),
                    (x_min + 1, y_min + 1),
                ];

                let check_validity = {
                    for (i, j) in pos_after_rotation {
                        if i < 0 || j > 19 {
                            return false;
                        }
                    }
                    true
                };

                check_validity
            }
            (
                Direction::RotPlus,
                crate::piece_type::PieceType::I,
                crate::orientation::Orientation::L,
            ) => {
                let pos_after_rotation = [
                    (x_min - 1, y_min + 2),
                    (x_min, y_min + 2),
                    (x_min + 1, y_min + 2),
                    (x_min + 2, y_min + 2),
                ];

                let check_validity = {
                    for (i, j) in pos_after_rotation {
                        if i < 0 || j > 19 {
                            return false;
                        }
                    }
                    true
                };

                check_validity
            }
            (Direction::RotPlus, crate::piece_type::PieceType::O, _) => true,
            (Direction::RotPlus, crate::piece_type::PieceType::T, _) => todo!(),
            (Direction::RotPlus, crate::piece_type::PieceType::J, _) => todo!(),
            (Direction::RotPlus, crate::piece_type::PieceType::L, _) => todo!(),
            (Direction::RotPlus, crate::piece_type::PieceType::S, _) => todo!(),
            (Direction::RotPlus, crate::piece_type::PieceType::Z, _) => todo!(),
            (Direction::RotMinus, crate::piece_type::PieceType::I, _) => todo!(),
            (Direction::RotMinus, crate::piece_type::PieceType::O, _) => true,
            (Direction::RotMinus, crate::piece_type::PieceType::T, _) => todo!(),
            (Direction::RotMinus, crate::piece_type::PieceType::J, _) => todo!(),
            (Direction::RotMinus, crate::piece_type::PieceType::L, _) => todo!(),
            (Direction::RotMinus, crate::piece_type::PieceType::S, _) => todo!(),
            (Direction::RotMinus, crate::piece_type::PieceType::Z, _) => todo!(),
        }
    }

    pub fn move_piece(&mut self, direction: Direction) {
        let will_move: bool = self.no_piece_around(&direction);
        if will_move {
            self.remove_piece();
            self.current_piece.piece_update_position(direction);
            self.place_piece(self.current_piece);
        }
    }

    fn drop_down_piece(mut self) -> Self {
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

    fn rotate_piece(mut self, direction: Direction) -> Self {
        self
    }
}

#[cfg(test)]
mod test {

    use std::cmp::min;

    use crate::{
        constants::{COLUMNS, LINES},
        orientation::Orientation,
        piece_position::PiecePosition,
        piece_type::PieceType,
    };

    use super::*;
    #[test]
    fn test_init() {
        let mut data = [[false; COLUMNS]; LINES];
        let piece_type: PieceType = rand::random();
        let piece = Piece::new(piece_type, PiecePosition::new(piece_type), Orientation::O);
        for (x, y) in piece.get_position().pos {
            data[x as usize][y as usize] = true;
        }
        assert_eq!(Field::new(piece).data, data)
    }

    #[test]
    fn test_move_piece_side() {
        let mut data = [[false; COLUMNS]; LINES];
        let piece_type: PieceType = rand::random();
        let piece: Piece = Piece::new(piece_type, PiecePosition::new(piece_type), Orientation::O);
        let mut field = Field::new(piece);
        for (x, y) in piece.get_position().pos {
            data[x as usize][(y + 1) as usize] = true;
        }
        field.move_piece(Direction::Right);
        assert_eq!(field.data, data)
    }

    #[test]
    fn test_drop_down_piece() {
        let mut data = [[false; COLUMNS]; LINES];
        let piece_type: PieceType = PieceType::O;
        let piece = Piece::new(piece_type, PiecePosition::new(piece_type), Orientation::O);
        let mut field = Field::new(piece);
        for (x, y) in piece.get_position().pos {
            data[(x + 1) as usize][y as usize] = true;
        }
        data[2][0] = true;
        field.data[2][0] = true;
        assert_eq!(field.drop_down_piece().data, data)
    }
}
