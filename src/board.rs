use arrayvec::ArrayVec;

use crate::direction::Direction;
use crate::tile::Tile;
use crate::constants::MAX_BOARD_SIZE;
use crate::tile_play::TilePlay;

#[derive(Debug, Clone)]
pub struct Board {
    pub tiles: ArrayVec<Tile, MAX_BOARD_SIZE>,
}

impl Board {
    pub fn is_empty(&self) -> bool {
        self.tiles.is_empty()
    }

    pub fn left(&self) -> Option<u8> {
        self.tiles.first().map(|t| t.left)
    }

    pub fn right(&self) -> Option<u8> {
        self.tiles.last().map(|t| t.right)
    }

    pub fn play(&mut self, tile_play: TilePlay) {
        if self.is_empty() {
            self.tiles.push(tile_play.tile);
            return;
        }

        match tile_play.direction {
            Direction::Left => {
                self.tiles.insert(0, tile_play.tile);
            }
            Direction::Right => {
                self.tiles.push(tile_play.tile);
            }
        };
    }
}
