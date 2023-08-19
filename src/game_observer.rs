use arrayvec::ArrayVec;

use crate::{tile::Tile, player::Player, board::Board, constants::BLOCK_STARTING_HAND_SIZE};

pub trait GameObserver {
    fn game_started(&mut self, opponent_tiles: ArrayVec<Tile, BLOCK_STARTING_HAND_SIZE>) {}
    fn opponent_played(&mut self, tile: Tile, board: &Board) {}
    fn opponent_drew(&mut self, tile: Tile) {}
    fn opponent_was_blocked(&mut self, board: &Board) {}
}
