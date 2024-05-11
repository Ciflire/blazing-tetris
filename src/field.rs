use sdl2::libc::{printf, useconds_t};

use crate::constants::{COLUMNS, LINES};
use crate::direction::Direction;
use crate::piece::Piece;
use crate::piece_position::PiecePosition;
use crate::piece_type::PieceType;

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
        let position_to_test = piece_to_move.piece_next_position(*direction);
        for (x_next_pos, y_next_pos) in position_to_test.pos {
            if x_next_pos < 0
                || x_next_pos >= LINES as i32
                || y_next_pos < 0
                || y_next_pos >= COLUMNS as i32
            {
                return false;
            }
            for piece in other_pieces {
                for (x_other_piece, y_other_piece) in piece.get_position().pos {
                    // out of bounds
                    // Overlapping
                    if x_next_pos == x_other_piece && y_next_pos == y_other_piece {
                        return false;
                    }
                }
            }
        }
        true
    }

    pub fn move_piece(&mut self, direction: Direction) {
        let will_move: bool = self.no_piece_around(&direction);
        if will_move {
            self.remove_piece();
            self.current_piece.piece_update_position(direction);
            self.place_piece(self.current_piece);
        } else if !will_move && direction == Direction::Down {
            self.old_pieces.push(self.current_piece);
            self.clear_lines();
            let new_piece_type: PieceType = rand::random();
            let new_piece = Piece::new(
                new_piece_type,
                PiecePosition::new(new_piece_type),
                crate::orientation::Orientation::O,
            );
            self.current_piece = new_piece;
        }
    }

    pub fn lines_to_clear(&self) -> Vec<usize> {
        let mut res = vec![];
        'outer: for i in 0..LINES {
            for j in 0..COLUMNS {
                if !self.data[i as usize][j as usize] {
                    continue 'outer;
                }
            }
            res.push(i);
        }
        res
    }

    pub fn clear_lines(&mut self) {
        let list_line_to_clear = self.lines_to_clear();
        println!("{:?}", list_line_to_clear);

        // if no line to clear exit function
        if list_line_to_clear.len() == 0 {
            return;
        }
        // Moving pieces down
        for piece in &mut self.old_pieces {
            for (i, (x, y)) in (0..4).zip(piece.get_position().pos) {
                if x != -1 && x < list_line_to_clear[0] as i32 {
                    piece.position.pos[i] = (x + list_line_to_clear.len() as i32, y);
                }
            }
        }
        // Deleting element of pieces that should no longer be rendered
        for elt in list_line_to_clear.clone() {
            for col in 0..COLUMNS {
                self.data[elt][col] = false;
            }
            for piece in &mut self.old_pieces {
                println!("{:?}", piece);
                for (i, (mut x, mut y)) in (0..4).zip(piece.get_position().pos) {
                    if (x as usize) == elt {
                        piece.position.pos[i] = (-1, -1);
                    }
                }
                println!("{:?}", piece);
            }
        }
        // Moving line above deletions
        for lines in (0..list_line_to_clear[0]).rev() {
            self.data[(lines + list_line_to_clear.len()) as usize] = self.data[lines];
        }
        // Reinitializing lines
        for line in 0..list_line_to_clear.len() {
            self.data[line] = [false; COLUMNS]
        }
    }
}

#[cfg(test)]
mod test {

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
        field.move_piece(Direction::Down);
        assert_eq!(field.data, data)
    }
}
