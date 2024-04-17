#[derive(Debug, Clone, Copy)]
pub struct Field {
    current_piece: (u32, u32),
    data: [[bool; 10]; 20],
}

impl Field {
    pub fn new(current_piece: (u32, u32), data: [[bool; 10]; 20]) -> Self {
        Self {
            current_piece,
            data,
        }
        .place_piece(current_piece)
    }

    pub fn place_piece(mut self, new_piece: (u32, u32)) -> Self {
        self.data[new_piece.0 as usize][new_piece.1 as usize] = true;
        self
    }

    pub fn side_move_piece(mut self, direction: &str) -> Self {
        if direction == "Left" {
            if self.current_piece.1 > 0 {
                self.data[self.current_piece.0 as usize][self.current_piece.1 as usize] = false;
                self.data[self.current_piece.0 as usize][(self.current_piece.1 as usize) - 1] =
                    true;
                self.current_piece = (self.current_piece.0, self.current_piece.1 - 1);
            } else {
                println!("Reached border")
            }
        }
        if direction == "Right" {
            if self.current_piece.1 < 9 {
                self.data[self.current_piece.0 as usize][self.current_piece.1 as usize] = false;
                self.data[self.current_piece.0 as usize][(self.current_piece.1 as usize) + 1] =
                    true;
                self.current_piece = (self.current_piece.0, self.current_piece.1 + 1)
            } else {
                println!("Reached border")
            }
        }
        self
    }

    pub fn drop_down_piece(mut self) -> Self {
        if self.current_piece.0 < 19 {
            // not already all the way down
            // no piece is under the one dropping
            if !self.data[(self.current_piece.0 as usize) + 1][self.current_piece.1 as usize] {
                // dropping the piece down
                self.data[self.current_piece.0 as usize][self.current_piece.1 as usize] = false;
                self.current_piece.0 = self.current_piece.0 + 1;
                self.data[self.current_piece.0 as usize][self.current_piece.1 as usize] = true;
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
        let mut data = [[false; 10]; 20];
        data[0][0] = true;
        assert_eq!(Field::new((0, 0), [[false; 10]; 20]).data, data)
    }
    #[test]
    fn test_place_piece() {
        let mut coordinates: (u32, u32);
        for _ in 0..100 {
            let empty_field: Field = Field {
                current_piece: (0, 0),
                data: [[false; 10]; 20],
            };
            coordinates = (
                rand::thread_rng().gen_range(0..20),
                rand::thread_rng().gen_range(0..10),
            );
            assert_eq!(
                Field::new(coordinates, [[false; 10]; 20]).data,
                empty_field.place_piece(coordinates).data
            )
        }
    }
    #[test]
    fn test_side_move_piece() {
        let mut coordinates: (u32, u32); // déclare la variable coordonnées
        for _ in 0..100 {
            coordinates = (
                rand::thread_rng().gen_range(0..20),
                rand::thread_rng().gen_range(0..10),
            ); // génère des coordonnées random
            let field_piece_placed: Field = Field::new(coordinates, [[false; 10]; 20]);
            let field_piece_moved: Field = field_piece_placed.side_move_piece("Left");
            println!("{:?}, {:?}", field_piece_placed, field_piece_moved);
            if coordinates.1 == 0 {
                println!("eq");
                assert_eq!(field_piece_moved.data, field_piece_placed.data);
            } else {
                println!("not equal");
                assert_ne!(field_piece_moved.data, field_piece_placed.data);
            }
        }
        for _ in 0..100 {
            coordinates = (
                rand::thread_rng().gen_range(0..20),
                rand::thread_rng().gen_range(0..10),
            ); // génère des coordonnées random
            let field_piece_placed: Field = Field::new(coordinates, [[false; 10]; 20]);
            let field_piece_moved: Field = field_piece_placed.side_move_piece("Right");
            println!("{:?}, {:?}", field_piece_placed, field_piece_moved);
            if coordinates.1 == 9 {
                println!("eq");
                assert_eq!(field_piece_moved.data, field_piece_placed.data);
            } else {
                println!("not equal");
                assert_ne!(field_piece_moved.data, field_piece_placed.data);
            }
        }
    }
    #[test]
    fn test_drop_down_piece() {
        let mut coordinates: (u32, u32);
        for _ in 0..100 {
            coordinates = (
                rand::thread_rng().gen_range(0..20),
                rand::thread_rng().gen_range(0..10),
            ); // génère des coordonnées random
            let field_piece_placed: Field = Field::new(coordinates, [[false; 10]; 20]);
            let field_piece_moved: Field = field_piece_placed.drop_down_piece();
            println!("{:?}, {:?}", field_piece_placed, field_piece_moved);
            if coordinates.0 == 19 {
                println!("eq");
                assert_eq!(field_piece_moved.data, field_piece_placed.data);
            } else {
                println!("not equal");
                assert_ne!(field_piece_moved.data, field_piece_placed.data);
            }
        }
    }
}
