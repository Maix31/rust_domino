pub struct PossibleHand {
	possible_tiles: Vec<bool>,
}

impl Default for PossibleHand {
	fn default() -> PossibleHand {
		PossibleHand { tiles: Vec::with_capacity(28) }
	}
}

impl PossibleHand {
	fn tile_to_index(tile: Tile) -> usize {
		let min = tile.left().min(tile.right());
		let max = tile.left().max(tile.right());
		((min * 7) - (min * (min + 1) / 2) + (max - min)) as usize
	}

	fn index_to_tile(index: usize) -> Tile {
		let min = ((index as u8) / 7) + 1;
		let max = ((index as u8) % 7) + min;
		Tile::new(min, max)
	}

	pub fn new() -> PossibleHand {
		PossibleHand { possible_tiles: vec![true; 28] }
	}

}
