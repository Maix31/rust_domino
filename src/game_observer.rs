use crate::{tile::Tile, snake::Snake};

pub trait GameObserver {
	fn game_started(&mut self) {}
	fn opponent_drew_tile(&mut self) {}
	fn opponent_played_tile(&mut self, tile: Tile) {}
	fn opponent_was_blocked(&mut self, pips: [u8;2]) {}
}
