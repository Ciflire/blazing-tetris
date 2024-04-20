use crate::piece_type::PieceType;

#[derive(Debug, Clone, Copy)]
pub struct PiecePosition {
    pub pos: [(u32, u32); 4],
}
impl PiecePosition {
    pub fn new(piece_type: PieceType) -> Self {
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
