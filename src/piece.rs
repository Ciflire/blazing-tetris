use crate::{
    direction::Direction, orientation::Orientation, piece_position::PiecePosition,
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
    fn _get_piece_type(&self) -> PieceType {
        self.piece_type
    }

    pub fn get_position(&self) -> PiecePosition {
        self.position
    }

    fn _get_orientation(&self) -> Orientation {
        self.orientation
    }

    pub fn piece_next_position(&self, direction: Direction) -> PiecePosition {
        let (x_min, y_min) = (
            self.get_position()
                .pos
                .map(|t| t.0)
                .iter()
                .min()
                .unwrap()
                .to_owned(),
            self.get_position()
                .pos
                .map(|t| t.1)
                .iter()
                .min()
                .unwrap()
                .to_owned(),
        );
        let mut res = PiecePosition { pos: [(0, 0); 4] };

        match (direction, self.piece_type, self.orientation) {
            (Direction::Left, _, _) => {
                for (i, (x, y)) in (0..4).zip(self.get_position().pos) {
                    res.pos[i] = (x, y - 1);
                }
                res
            }

            (Direction::Right, _, _) => {
                for (i, (x, y)) in (0..4).zip(self.get_position().pos) {
                    res.pos[i] = (x, y + 1);
                }
                res
            }
            (Direction::Down, _, _) => {
                for (i, (x, y)) in (0..4).zip(self.get_position().pos) {
                    res.pos[i] = (x + 1, y);
                }
                res
            }
            (Direction::RotPlus, PieceType::I, Orientation::O) => PiecePosition {
                pos: [
                    (x_min - 1, y_min + 2),
                    (x_min, y_min + 2),
                    (x_min + 1, y_min + 2),
                    (x_min + 2, y_min + 2),
                ],
            },
            (Direction::RotPlus, PieceType::I, Orientation::R) => PiecePosition {
                pos: [
                    (x_min + 2, y_min - 2),
                    (x_min + 2, y_min - 1),
                    (x_min + 2, y_min),
                    (x_min + 2, y_min + 1),
                ],
            },
            (Direction::RotPlus, PieceType::I, Orientation::Half) => PiecePosition {
                pos: [
                    (x_min - 2, y_min + 1),
                    (x_min - 1, y_min + 1),
                    (x_min, y_min + 1),
                    (x_min + 1, y_min + 1),
                ],
            },
            (Direction::RotPlus, PieceType::I, Orientation::L) => PiecePosition {
                pos: [
                    (x_min + 1, y_min - 1),
                    (x_min + 1, y_min),
                    (x_min + 1, y_min + 1),
                    (x_min + 1, y_min + 2),
                ],
            },
            (Direction::RotPlus, PieceType::O, Orientation::O) => self.get_position(),
            (Direction::RotPlus, PieceType::O, Orientation::R) => self.get_position(),
            (Direction::RotPlus, PieceType::O, Orientation::Half) => self.get_position(),
            (Direction::RotPlus, PieceType::O, Orientation::L) => self.get_position(),
            (Direction::RotPlus, PieceType::T, Orientation::O) => PiecePosition {
                pos: [
                    (x_min, y_min + 1),
                    (x_min + 1, y_min + 1),
                    (x_min + 1, y_min + 2),
                    (x_min + 2, y_min + 1),
                ],
            },
            (Direction::RotPlus, PieceType::T, Orientation::R) => PiecePosition {
                pos: [
                    (x_min + 1, y_min - 1),
                    (x_min + 1, y_min),
                    (x_min + 1, y_min + 1),
                    (x_min + 2, y_min),
                ],
            },
            (Direction::RotPlus, PieceType::T, Orientation::Half) => PiecePosition {
                pos: [
                    (x_min, y_min),
                    (x_min - 1, y_min + 1),
                    (x_min, y_min + 1),
                    (x_min + 1, y_min + 1),
                ],
            },
            (Direction::RotPlus, PieceType::T, Orientation::L) => PiecePosition {
                pos: [
                    (x_min, y_min + 1),
                    (x_min + 1, y_min),
                    (x_min + 1, y_min + 1),
                    (x_min + 1, y_min + 2),
                ],
            },
            (Direction::RotPlus, PieceType::J, Orientation::O) => PiecePosition {
                pos: [
                    (x_min, y_min + 1),
                    (x_min, y_min + 2),
                    (x_min + 1, y_min + 1),
                    (x_min + 2, y_min + 1),
                ],
            },
            (Direction::RotPlus, PieceType::J, Orientation::R) => PiecePosition {
                pos: [
                    (x_min + 1, y_min - 1),
                    (x_min + 1, y_min),
                    (x_min + 1, y_min + 1),
                    (x_min + 2, y_min + 1),
                ],
            },
            (Direction::RotPlus, PieceType::J, Orientation::Half) => PiecePosition {
                pos: [
                    (x_min - 1, y_min + 1),
                    (x_min, y_min + 1),
                    (x_min + 1, y_min),
                    (x_min + 1, y_min + 1),
                ],
            },
            (Direction::RotPlus, PieceType::J, Orientation::L) => PiecePosition {
                pos: [
                    (x_min, y_min),
                    (x_min + 1, y_min),
                    (x_min + 1, y_min + 1),
                    (x_min + 1, y_min + 2),
                ],
            },
            (Direction::RotPlus, PieceType::L, Orientation::O) => PiecePosition {
                pos: [
                    (x_min, y_min + 1),
                    (x_min, y_min + 2),
                    (x_min + 1, y_min + 1),
                    (x_min + 2, y_min + 1),
                ],
            },
            (Direction::RotPlus, PieceType::L, Orientation::R) => PiecePosition {
                pos: [
                    (x_min + 1, y_min - 1),
                    (x_min + 1, y_min),
                    (x_min + 1, y_min + 1),
                    (x_min + 2, y_min + 1),
                ],
            },
            (Direction::RotPlus, PieceType::L, Orientation::Half) => PiecePosition {
                pos: [
                    (x_min - 1, y_min + 1),
                    (x_min, y_min + 1),
                    (x_min + 1, y_min),
                    (x_min + 1, y_min + 1),
                ],
            },
            (Direction::RotPlus, PieceType::L, Orientation::L) => PiecePosition {
                pos: [
                    (x_min, y_min),
                    (x_min + 1, y_min),
                    (x_min + 1, y_min + 1),
                    (x_min + 1, y_min + 2),
                ],
            },
            (Direction::RotPlus, PieceType::S, Orientation::O) => PiecePosition {
                pos: [
                    (x_min, y_min + 1),
                    (x_min + 1, y_min + 1),
                    (x_min + 1, y_min + 2),
                    (x_min + 2, y_min + 2),
                ],
            },
            (Direction::RotPlus, PieceType::S, Orientation::R) => PiecePosition {
                pos: [
                    (x_min + 1, y_min),
                    (x_min + 1, y_min + 1),
                    (x_min + 2, y_min - 1),
                    (x_min + 2, y_min),
                ],
            },
            (Direction::RotPlus, PieceType::S, Orientation::Half) => PiecePosition {
                pos: [
                    (x_min - 1, y_min),
                    (x_min, y_min),
                    (x_min, y_min + 1),
                    (x_min + 1, y_min + 1),
                ],
            },
            (Direction::RotPlus, PieceType::S, Orientation::L) => PiecePosition {
                pos: [
                    (x_min, y_min + 1),
                    (x_min, y_min + 2),
                    (x_min + 1, y_min),
                    (x_min + 1, y_min + 1),
                ],
            },
            (Direction::RotPlus, PieceType::Z, Orientation::O) => PiecePosition {
                pos: [
                    (x_min, y_min + 2),
                    (x_min + 1, y_min + 1),
                    (x_min + 1, y_min + 2),
                    (x_min + 2, y_min + 1),
                ],
            },
            (Direction::RotPlus, PieceType::Z, Orientation::R) => PiecePosition {
                pos: [
                    (x_min + 1, y_min - 1),
                    (x_min + 1, y_min),
                    (x_min + 2, y_min),
                    (x_min + 2, y_min + 1),
                ],
            },
            (Direction::RotPlus, PieceType::Z, Orientation::Half) => PiecePosition {
                pos: [
                    (x_min - 1, y_min + 1),
                    (x_min, y_min),
                    (x_min, y_min + 1),
                    (x_min + 1, y_min),
                ],
            },
            (Direction::RotPlus, PieceType::Z, Orientation::L) => PiecePosition {
                pos: [
                    (x_min, y_min),
                    (x_min, y_min + 1),
                    (x_min + 1, y_min + 1),
                    (x_min + 1, y_min + 2),
                ],
            },
            (Direction::RotMinus, PieceType::I, Orientation::O) => PiecePosition {
                pos: [
                    (x_min - 1, y_min + 1),
                    (x_min, y_min + 1),
                    (x_min + 1, y_min + 1),
                    (x_min + 2, y_min + 1),
                ],
            },
            (Direction::RotMinus, PieceType::I, Orientation::R) => PiecePosition {
                pos: [
                    (x_min + 1, y_min - 2),
                    (x_min + 1, y_min - 1),
                    (x_min + 1, y_min),
                    (x_min + 1, y_min + 1),
                ],
            },
            (Direction::RotMinus, PieceType::I, Orientation::Half) => PiecePosition {
                pos: [
                    (x_min - 2, y_min + 2),
                    (x_min - 1, y_min + 2),
                    (x_min, y_min + 2),
                    (x_min + 1, y_min + 2),
                ],
            },
            (Direction::RotMinus, PieceType::I, Orientation::L) => PiecePosition {
                pos: [
                    (x_min + 2, y_min - 1),
                    (x_min + 2, y_min),
                    (x_min + 2, y_min + 1),
                    (x_min + 2, y_min + 2),
                ],
            },
            (Direction::RotMinus, PieceType::O, Orientation::O) => self.get_position(),
            (Direction::RotMinus, PieceType::O, Orientation::R) => self.get_position(),
            (Direction::RotMinus, PieceType::O, Orientation::Half) => self.get_position(),
            (Direction::RotMinus, PieceType::O, Orientation::L) => self.get_position(),
            (Direction::RotMinus, PieceType::T, Orientation::O) => PiecePosition {
                pos: [
                    (x_min, y_min + 1),
                    (x_min + 1, y_min),
                    (x_min + 1, y_min + 1),
                    (x_min + 2, y_min + 1),
                ],
            },
            (Direction::RotMinus, PieceType::T, Orientation::R) => PiecePosition {
                pos: [
                    (x_min, y_min),
                    (x_min + 1, y_min - 1),
                    (x_min + 1, y_min),
                    (x_min + 1, y_min + 1),
                ],
            },
            (Direction::RotMinus, PieceType::T, Orientation::Half) => PiecePosition {
                pos: [
                    (x_min - 1, y_min + 1),
                    (x_min, y_min + 1),
                    (x_min, y_min + 2),
                    (x_min + 1, y_min + 1),
                ],
            },
            (Direction::RotMinus, PieceType::T, Orientation::L) => PiecePosition {
                pos: [
                    (x_min + 1, y_min),
                    (x_min + 1, y_min + 1),
                    (x_min + 1, y_min + 2),
                    (x_min + 2, y_min + 1),
                ],
            },
            (Direction::RotMinus, PieceType::J, Orientation::O) => PiecePosition {
                pos: [
                    (x_min, y_min + 1),
                    (x_min + 1, y_min + 1),
                    (x_min + 2, y_min + 1),
                    (x_min + 2, y_min),
                ],
            },
            (Direction::RotMinus, PieceType::J, Orientation::R) => PiecePosition {
                pos: [
                    (x_min, y_min - 1),
                    (x_min + 1, y_min - 1),
                    (x_min + 1, y_min),
                    (x_min + 1, y_min + 1),
                ],
            },
            (Direction::RotMinus, PieceType::J, Orientation::Half) => PiecePosition {
                pos: [
                    (x_min - 1, y_min + 1),
                    (x_min, y_min + 1),
                    (x_min + 1, y_min + 1),
                    (x_min - 1, y_min + 2),
                ],
            },
            (Direction::RotMinus, PieceType::J, Orientation::L) => PiecePosition {
                pos: [
                    (x_min + 1, y_min),
                    (x_min + 1, y_min + 1),
                    (x_min + 1, y_min + 2),
                    (x_min + 2, y_min + 2),
                ],
            },
            (Direction::RotMinus, PieceType::L, Orientation::O) => PiecePosition {
                pos: [
                    (x_min, y_min),
                    (x_min, y_min + 1),
                    (x_min + 1, y_min + 1),
                    (x_min + 2, y_min + 1),
                ],
            },
            (Direction::RotMinus, PieceType::L, Orientation::R) => PiecePosition {
                pos: [
                    (x_min, y_min + 1),
                    (x_min + 1, y_min - 1),
                    (x_min + 1, y_min),
                    (x_min + 1, y_min + 1),
                ],
            },
            (Direction::RotMinus, PieceType::L, Orientation::Half) => PiecePosition {
                pos: [
                    (x_min - 1, y_min + 1),
                    (x_min, y_min + 1),
                    (x_min + 1, y_min + 1),
                    (x_min + 1, y_min + 2),
                ],
            },
            (Direction::RotMinus, PieceType::L, Orientation::L) => PiecePosition {
                pos: [
                    (x_min + 1, y_min),
                    (x_min + 1, y_min + 1),
                    (x_min + 1, y_min + 2),
                    (x_min + 2, y_min),
                ],
            },
            (Direction::RotMinus, PieceType::S, Orientation::O) => PiecePosition {
                pos: [
                    (x_min, y_min),
                    (x_min + 1, y_min),
                    (x_min + 1, y_min + 1),
                    (x_min + 2, y_min + 1),
                ],
            },
            (Direction::RotMinus, PieceType::S, Orientation::R) => PiecePosition {
                pos: [
                    (x_min, y_min),
                    (x_min, y_min + 1),
                    (x_min + 1, y_min - 1),
                    (x_min + 1, y_min),
                ],
            },
            (Direction::RotMinus, PieceType::S, Orientation::Half) => PiecePosition {
                pos: [
                    (x_min - 1, y_min + 1),
                    (x_min, y_min + 1),
                    (x_min, y_min + 2),
                    (x_min + 1, y_min + 2),
                ],
            },
            (Direction::RotMinus, PieceType::S, Orientation::L) => PiecePosition {
                pos: [
                    (x_min + 1, y_min + 1),
                    (x_min + 1, y_min + 2),
                    (x_min + 2, y_min),
                    (x_min + 2, y_min + 1),
                ],
            },
            (Direction::RotMinus, PieceType::Z, Orientation::O) => PiecePosition {
                pos: [
                    (x_min, y_min + 1),
                    (x_min + 1, y_min),
                    (x_min + 1, y_min + 1),
                    (x_min + 2, y_min),
                ],
            },
            (Direction::RotMinus, PieceType::Z, Orientation::R) => PiecePosition {
                pos: [
                    (x_min, y_min - 1),
                    (x_min, y_min),
                    (x_min + 1, y_min),
                    (x_min + 1, y_min + 1),
                ],
            },
            (Direction::RotMinus, PieceType::Z, Orientation::Half) => PiecePosition {
                pos: [
                    (x_min - 1, y_min + 2),
                    (x_min, y_min + 1),
                    (x_min, y_min + 2),
                    (x_min + 1, y_min + 1),
                ],
            },
            (Direction::RotMinus, PieceType::Z, Orientation::L) => PiecePosition {
                pos: [
                    (x_min + 1, y_min),
                    (x_min + 1, y_min + 1),
                    (x_min + 2, y_min + 1),
                    (x_min + 2, y_min + 2),
                ],
            },
        }
    }

    pub fn piece_update_position(&mut self, direction: Direction) {
        let next_position = self.piece_next_position(direction);
        self.position = next_position;
        match (self.orientation, direction) {
            (Orientation::O, Direction::RotPlus) => self.orientation = Orientation::R,
            (Orientation::O, Direction::RotMinus) => self.orientation = Orientation::L,
            (Orientation::R, Direction::RotPlus) => self.orientation = Orientation::Half,
            (Orientation::R, Direction::RotMinus) => self.orientation = Orientation::O,
            (Orientation::Half, Direction::RotPlus) => self.orientation = Orientation::L,
            (Orientation::Half, Direction::RotMinus) => self.orientation = Orientation::R,
            (Orientation::L, Direction::RotPlus) => self.orientation = Orientation::O,
            (Orientation::L, Direction::RotMinus) => self.orientation = Orientation::Half,
            _ => (),
        };
    }
}
