#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tile {
    pub left: u8,
    pub right: u8,
}

impl Tile {
    pub fn flip(self) -> Tile {
        Tile {
            left: self.right,
            right: self.left,
        }
    }
}
