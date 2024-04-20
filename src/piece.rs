use crate::{orientation::Orientation, piece_position::PiecePosition, piece_type::PieceType};

#[derive(Debug, Clone, Copy)]
pub struct Piece {
    piece_type: PieceType,
    pub position: PiecePosition,
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

    pub fn get_position(&self) -> PiecePosition {
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
