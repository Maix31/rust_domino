use crate::{hand::{Hand, HasHandTrait}, snake::Snake, tile::Tile, player::Player, choose_tile_strategy::ChooseTileStrategy, game_observer::GameObserver};

#[derive(Default)]
pub struct FirstPossibleTileAIPlayer {
	pub hand: Hand,
}

impl Player for FirstPossibleTileAIPlayer {}

impl HasHandTrait for FirstPossibleTileAIPlayer {
    fn hand(&self) -> &Hand {
        &self.hand
    }
    fn hand_mut(&mut self) -> &mut Hand {
        &mut self.hand
    }
}

impl ChooseTileStrategy for  FirstPossibleTileAIPlayer {
    fn choose_tile(&mut self, snake: &Snake) -> Tile {
        let tile_index = self.hand.tiles.iter().position(|tile| snake.is_playable(*tile)).unwrap();
        self.hand.tiles.remove(tile_index)
    }
}

impl GameObserver for FirstPossibleTileAIPlayer {}
