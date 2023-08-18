use arrayvec::ArrayVec;
use bitarray::{self, BitArray};

use crate::tile::Tile;

pub struct PossibleHand {
	// this holds a bit array of all possible tiles 
	pub possible_tiles: ArrayVec<Tile, 28>,
	pub size: u8,

}

impl PossibleHand {
	// fn tile_to_index(tile: Tile) -> usize {
	// 	let min = tile.left.min(tile.right);
	// 	let max = tile.left.max(tile.right);
	// 	((min * 7) - (min * (min + 1) / 2) + (max - min)) as usize
	// }

	// fn index_to_tile(index: usize) -> Tile {
	// 	let min = ((index as u8) / 7) + 1;
	// 	let max = ((index as u8) % 7) + min;
	// 	Tile::new(min, max)
	// }

	pub fn new() -> PossibleHand {
		let mut tiles = ArrayVec::new();
        for left in 0..7 {
            for right in left..7 {
                tiles.push(Tile::new(left, right));
            }
        }
		PossibleHand { possible_tiles: tiles, size: 0 }
	}

	pub fn remove_tile(&mut self, tile: Tile) {
		let index = self.possible_tiles.iter().position(|t| *t == tile).unwrap();
		self.possible_tiles.remove(index);
	}

	pub fn remove_tiles_with_pips(&mut self, pips: [u8; 2] ) {
		let mut index = 0;
		while index < self.possible_tiles.len() {
			let tile = self.possible_tiles[index];
			if tile.left == pips[0] || tile.right == pips[0] || tile.left == pips[1] || tile.right == pips[1] {
				self.possible_tiles.remove(index);
			} else {
				index += 1;
			}
		}
	}

}

// make a  test
#[cfg(test)]
mod tests {
    use crate::tile::Tile;

	#[test]
	fn test_possible_hand() {
		let mut tiles = [ 
			Tile::new(0,0), Tile::new(0,1), Tile::new(0,2), Tile::new(0,3), Tile::new(0,4), Tile::new(0,5), Tile::new(0,6),
			Tile::new(1,1), Tile::new(1,2), Tile::new(1,3), Tile::new(1,4), Tile::new(1,5), Tile::new(1,6),
			Tile::new(2,2), Tile::new(2,3), Tile::new(2,4), Tile::new(2,5), Tile::new(2,6),
			Tile::new(3,3), Tile::new(3,4), Tile::new(3,5), Tile::new(3,6),
			Tile::new(4,4), Tile::new(4,5), Tile::new(4,6),
			Tile::new(5,5), Tile::new(5,6),
			Tile::new(6,6)
		];

		assert!(tiles.len() == 28);

		for i in 0..tiles.len() {
			// let tile = tiles[i];
			// let index = super::PossibleHand::tile_to_index(tile);
			// assert!(index == i);
			// let tile2 = super::PossibleHand::index_to_tile(index);
			// assert!(tile == tile2);
		}
	}
}

impl Default for PossibleHand {
	fn default() -> Self {
		Self::new()
	}
}
