use crate::{hand::Hand, snake::Snake, tile::Tile};

enum PossibleTile {
    Yes, No, Maybe
}

struct PossibleHand {
    tiles: [PossibleTile; 28],
    tiles_count: u8,
}

pub trait ChooseTileStrategy {
	fn choose_tile(&mut self, snake: &Snake) -> Tile;
}
