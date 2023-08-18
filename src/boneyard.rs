use arrayvec::ArrayVec;
use rand::{seq::SliceRandom, rngs::ThreadRng};

use crate::tile::Tile;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Boneyard {
    pub tiles: ArrayVec<Tile, 28>,
}

impl Boneyard {
    pub fn new() -> Boneyard {
        let mut tiles = ArrayVec::new();
        for left in 0..7 {
            for right in left..7 {
                tiles.push(Tile::new(left, right));
            }
        }
        Boneyard { tiles }
    }

    pub fn shuffle(mut self, rng: &mut ThreadRng) -> Self {
        self.tiles.shuffle(rng);
        self
    }

    pub fn draw_n(&mut self, n: i32) -> ArrayVec<Tile, 7> {
        let tiles_count = self.tiles.len();
        // remove the top n tiles
        let tiles = self.tiles.drain(tiles_count - n as usize..).collect();
        tiles
    }

    pub fn draw(&mut self) -> Tile {
        self.tiles.pop().unwrap()
    }

    pub fn is_empty(&self) -> bool {
        self.tiles.len() == 0
    }
}
