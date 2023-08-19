use crate::{tile::Tile, direction::Direction};

#[derive(Clone, Copy, Debug)]
pub struct TilePlay {
    pub tile: Tile,
    pub direction: Direction,
}
