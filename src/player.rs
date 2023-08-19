use arrayvec::ArrayVec;

use crate::{
    constants::BLOCK_STARTING_HAND_SIZE, game_observer::GameObserver, tile::Tile,
    tile_play::TilePlay, board::Board,
};

pub trait Player: GameObserver {
    fn play_tile(&mut self, left: Option<u8>, right: Option<u8>, board: &Board)-> Option<TilePlay>;
    fn draw_tiles(&mut self, starting_hand: ArrayVec<Tile, BLOCK_STARTING_HAND_SIZE>);
    fn hand(&self) -> &[Tile];
    fn box_clone(&self) -> Box<dyn Player>;
    fn hand_sum(&self) -> u8;
    fn name(&self) -> &str;
}
