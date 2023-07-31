use crate::{hand::{Hand, HasHandTrait}, snake::Snake, tile::Tile, player::Player, choose_tile_strategy::ChooseTileStrategy, game_observer::GameObserver};

#[derive(Default)]
pub struct ObserventAIPlayer {
	pub hand: Hand,
}

impl Player for ObserventAIPlayer {}

impl HasHandTrait for ObserventAIPlayer {
    fn hand(&self) -> &Hand {
        &self.hand
    }
    fn hand_mut(&mut self) -> &mut Hand {
        &mut self.hand
    }
}

impl ChooseTileStrategy for ObserventAIPlayer {
    fn choose_tile(&mut self, snake: &Snake) -> Tile {
        let tile_index = self.hand.tiles.iter().position(|tile| snake.is_playable(*tile)).unwrap();
        self.hand.tiles.remove(tile_index)
    }
}

impl GameObserver for ObserventAIPlayer {}
