use crate::{hand::{Hand, HasHandTrait}, snake::Snake, tile::Tile, player::Player, choose_tile_strategy::ChooseTileStrategy, game_observer::GameObserver, possible_hand::PossibleHand};

#[derive(Default)]
pub struct ObserventAIPlayer {
	pub hand: Hand,
    opponent_hand: PossibleHand,
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

impl GameObserver for ObserventAIPlayer {
    // called after deal
    fn game_started(&mut self) {
        // remove my tiles from opponent's possible hand
        for tile in self.hand.tiles.iter() {
            self.opponent_hand.remove_tile(*tile);
        }
        self.opponent_hand.size = self.hand.tiles.len() as u8;
    }

	fn opponent_drew_tile(&mut self) {
        self.opponent_hand.size += 1;
    }

    fn i_drew_tile(&mut self, tile: Tile) {
        // remove my tile from opponent's possible hand
        self.opponent_hand.remove_tile(tile);
    }

	fn opponent_played_tile(&mut self, tile: Tile) {
        self.opponent_hand.remove_tile(tile);
        self.opponent_hand.size -= 1;

        println!("opponents possible hand size: {}", self.opponent_hand.size);
        println!("opponents possible hand: {:?}", self.opponent_hand.possible_tiles);
        println!("opponents possible hand: {:?}", self.opponent_hand.possible_tiles.len());
    }

	fn opponent_was_blocked(&mut self, pips: [u8;2]) {
        self.opponent_hand.remove_tiles_with_pips(pips);
    }
}
