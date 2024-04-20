use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

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
