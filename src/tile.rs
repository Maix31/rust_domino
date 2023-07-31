use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tile {
    pub left: u8,
    pub right: u8,
}

impl Tile {
    pub fn new(left: u8, right: u8) -> Tile {
        Tile { left, right }
    }

    pub fn flip(&self) -> Tile {
        Tile {
            left: self.right,
            right: self.left,
        }
    }

    pub fn score(&self) -> u8 {
        self.left + self.right
    }
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}|{}]", self.left, self.right)
    }
}
