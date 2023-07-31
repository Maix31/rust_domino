use std::fmt::Debug;

use crate::tile::Tile;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Snake {
    pub tiles: Vec<Tile>,
}

impl Snake {
    pub fn new() -> Snake {
        Snake { tiles: Vec::with_capacity(21) }
    }

    pub fn left(&self) -> Option<u8> {
        self.tiles.first().map(|tile| tile.left)
    }

    pub fn right(&self) -> Option<u8> {
        self.tiles.last().map(|tile| tile.right)
    }

    pub fn is_playable(&self, tile: Tile) -> bool {
        self.left().is_none()
            || self.left() == Some(tile.left)
            || self.left() == Some(tile.right)
            || self.right() == Some(tile.left)
            || self.right() == Some(tile.right)
    }

    pub fn is_empty(&self) -> bool {
        self.tiles.is_empty()
    }

    /// the tile should be playeble
    /// the tile is playable if it can be placed on the left or right side of the snake
    pub fn add(&mut self, tile: Tile) {
        if self.tiles.is_empty() {
            self.tiles.push(tile);
        } else if self.left() == Some(tile.right) {
            self.tiles.insert(0, tile);
        } else if self.left() == Some(tile.left) {
            self.tiles.insert(0, tile.flip());
        } else if self.right() == Some(tile.left) {
            self.tiles.push(tile);
        } else if self.right() == Some(tile.right) {
            self.tiles.push(tile.flip());
        }

        assert!(self.is_valid());
    }

    fn is_valid(&self) -> bool {
        if self.tiles.is_empty() {
            return true;
        }

        let mut prev = self.tiles.first().unwrap();
        for tile in self.tiles.iter().skip(1) {
            if prev.right != tile.left {
                return false;
            }
            prev = tile;
        }
        true
    }
}
