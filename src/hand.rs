use crate::{tile::Tile, boneyard::{Boneyard, self}, snake::Snake};

pub trait TilesTrait {
    fn tiles(&self) -> &Vec<Tile>;
    fn tiles_mut(&mut self) -> &mut Vec<Tile>;
}

pub trait HandTrait: TilesTrait {
    fn is_empty(&self) -> bool {
        self.tiles().is_empty()
    }

    fn add(&mut self, tile: Tile) {
        self.tiles_mut().push(tile);
    }

    fn add_multiple(&mut self, tiles: Vec<Tile>) {
        self.tiles_mut().extend(tiles);
    }

    fn score(&self) -> i32 {
        self.tiles().iter().map(|tile| tile.score() as i32).sum()
    }
}

pub trait HasHandTrait {
    fn hand(&self) -> &Hand;
    fn hand_mut(&mut self) -> &mut Hand;
}

pub struct Hand {
    pub tiles: Vec<Tile>,
}

impl Default for Hand {
    fn default() -> Hand {
        Hand { tiles: Vec::with_capacity(21) }
    }
}

impl HandTrait for Hand {}

impl TilesTrait for Hand {
    fn tiles(&self) -> &Vec<Tile> {
        &self.tiles
    }
    fn tiles_mut(&mut self) -> &mut Vec<Tile> {
        &mut self.tiles
    }
}
