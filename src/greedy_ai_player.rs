use crate::{hand::{Hand, HasHandTrait}, snake::Snake, tile::Tile, player::Player, choose_tile_strategy::ChooseTileStrategy, game_observer::GameObserver};

#[derive(Default)]
pub struct GreedyAIPlayer {
	pub hand: Hand,
}

impl Player for GreedyAIPlayer {}

impl HasHandTrait for GreedyAIPlayer {
    fn hand(&self) -> &Hand {
        &self.hand
    }
    fn hand_mut(&mut self) -> &mut Hand {
        &mut self.hand
    }
}

impl ChooseTileStrategy for  GreedyAIPlayer {
    fn choose_tile(&mut self, snake: &Snake) -> Tile {
        let tile_index = self.hand.tiles.iter().filter(|tile| snake.is_playable(**tile)).enumerate().max_by_key(|(_, tile)| tile.score()).unwrap().0;
        self.hand.tiles.remove(tile_index)
    }
}

impl GameObserver for GreedyAIPlayer {}
