use crate::{snake::Snake, tile::Tile, hand::{Hand, HasHandTrait}, choose_tile_strategy::ChooseTileStrategy, game_observer::GameObserver, player::Player};

#[derive(Default)]
pub struct HumanPlayer {
	pub hand: Hand,
}

impl HasHandTrait for HumanPlayer {
    fn hand(&self) -> &Hand {
        &self.hand
    }
    fn hand_mut(&mut self) -> &mut Hand {
        &mut self.hand
    }
}

impl ChooseTileStrategy for  HumanPlayer {
	fn choose_tile(&mut self, snake: &Snake) -> Tile {
        // get console input from stdin
        let mut input = String::new();
        println!("Choose a tile from your hand");
        while let Err(_) = std::io::stdin().read_line(&mut input) {
            println!("Invalid input, try again");
            input.clear();
        }

        let index = input.trim().parse::<usize>();

        let index = match index {
            Ok(index) => index,
            Err(_) => {
                println!("Invalid input, try again");
                return self.choose_tile(snake);
            }
        };

        let tile = self.hand.tiles.get(index).map(|f| *f);

        let tile_is_playable = match tile {
            Some(tile) => snake.is_playable(tile),
            None => false,
        };

        if !tile_is_playable {
            println!("Invalid tile, try again");
            return self.choose_tile(snake);
        }

        println!("You chose {:?}", tile);

        let tile = tile.unwrap();

        let index = self.hand.tiles.iter().position(|t| *t == tile).unwrap();
        self.hand.tiles.remove(index)
    }
}

impl Player for HumanPlayer {}

impl GameObserver for HumanPlayer {}
