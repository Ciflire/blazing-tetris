use crate::{
    direction::{self, Direction},
    orientation::Orientation,
    piece_position::PiecePosition,
    piece_type::PieceType,
};

#[derive(Debug, Clone, Copy)]
pub struct Piece {
    pub piece_type: PieceType,
    pub position: PiecePosition,
    pub orientation: Orientation,
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

    pub fn piece_update_position(&mut self, direction: Direction) {
        let (x_min, y_min) = (
            self.get_position()
                .pos
                .map(|t| t.0)
                .iter()
                .min()
                .unwrap()
                .to_owned() as u32,
            self.get_position()
                .pos
                .map(|t| t.1)
                .iter()
                .min()
                .unwrap()
                .to_owned() as u32,
        );
        match direction {
            Direction::Left => {
                for i in 0..4 {
                    self.position.pos[i as usize].1 -= 1;
                }
            }
            Direction::Right => {
                for i in 0..4 {
                    self.position.pos[i as usize].1 += 1;
                }
            }
            Direction::Down => {
                for i in 0..4 {
                    self.position.pos[i as usize].0 += 1;
                }
            }
            Direction::RotPlus => match (self.piece_type, self.orientation) {
                (PieceType::I, Orientation::O) => {
                    self.position = PiecePosition {
                        pos: [
                            (x_min - 1, y_min + 2),
                            (x_min, y_min + 2),
                            (x_min + 1, y_min + 2),
                            (x_min + 2, y_min + 2),
                        ],
                    };
                    self.orientation = crate::orientation::Orientation::R;
                }
                (PieceType::I, Orientation::R) => todo!(),
                (PieceType::I, Orientation::Half) => todo!(),
                (PieceType::I, Orientation::L) => todo!(),
                (PieceType::O, Orientation::O) => todo!(),
                (PieceType::O, Orientation::R) => todo!(),
                (PieceType::O, Orientation::Half) => todo!(),
                (PieceType::O, Orientation::L) => todo!(),
                (PieceType::T, Orientation::O) => todo!(),
                (PieceType::T, Orientation::R) => todo!(),
                (PieceType::T, Orientation::Half) => todo!(),
                (PieceType::T, Orientation::L) => todo!(),
                (PieceType::J, Orientation::O) => todo!(),
                (PieceType::J, Orientation::R) => todo!(),
                (PieceType::J, Orientation::Half) => todo!(),
                (PieceType::J, Orientation::L) => todo!(),
                (PieceType::L, Orientation::O) => todo!(),
                (PieceType::L, Orientation::R) => todo!(),
                (PieceType::L, Orientation::Half) => todo!(),
                (PieceType::L, Orientation::L) => todo!(),
                (PieceType::S, Orientation::O) => todo!(),
                (PieceType::S, Orientation::R) => todo!(),
                (PieceType::S, Orientation::Half) => todo!(),
                (PieceType::S, Orientation::L) => todo!(),
                (PieceType::Z, Orientation::O) => todo!(),
                (PieceType::Z, Orientation::R) => todo!(),
                (PieceType::Z, Orientation::Half) => todo!(),
                (PieceType::Z, Orientation::L) => todo!(),
            },
            Direction::RotMinus => match (self.piece_type, self.orientation) {
                (PieceType::I, Orientation::O) => todo!(),
                (PieceType::I, Orientation::R) => todo!(),
                (PieceType::I, Orientation::Half) => todo!(),
                (PieceType::I, Orientation::L) => todo!(),
                (PieceType::O, Orientation::O) => todo!(),
                (PieceType::O, Orientation::R) => todo!(),
                (PieceType::O, Orientation::Half) => todo!(),
                (PieceType::O, Orientation::L) => todo!(),
                (PieceType::T, Orientation::O) => todo!(),
                (PieceType::T, Orientation::R) => todo!(),
                (PieceType::T, Orientation::Half) => todo!(),
                (PieceType::T, Orientation::L) => todo!(),
                (PieceType::J, Orientation::O) => todo!(),
                (PieceType::J, Orientation::R) => todo!(),
                (PieceType::J, Orientation::Half) => todo!(),
                (PieceType::J, Orientation::L) => todo!(),
                (PieceType::L, Orientation::O) => todo!(),
                (PieceType::L, Orientation::R) => todo!(),
                (PieceType::L, Orientation::Half) => todo!(),
                (PieceType::L, Orientation::L) => todo!(),
                (PieceType::S, Orientation::O) => todo!(),
                (PieceType::S, Orientation::R) => todo!(),
                (PieceType::S, Orientation::Half) => todo!(),
                (PieceType::S, Orientation::L) => todo!(),
                (PieceType::Z, Orientation::O) => todo!(),
                (PieceType::Z, Orientation::R) => todo!(),
                (PieceType::Z, Orientation::Half) => todo!(),
                (PieceType::Z, Orientation::L) => todo!(),
            },
        }
    }

    fn check_down(&self) -> Vec<(u32, u32)> {
        let res: Vec<(u32, u32)>;
        match self.piece_type {
            PieceType::I => todo!(),
            PieceType::O => todo!(),
            PieceType::T => todo!(),
            PieceType::J => todo!(),
            PieceType::L => todo!(),
            PieceType::S => todo!(),
            PieceType::Z => todo!(),
        }
        res
    }
}
