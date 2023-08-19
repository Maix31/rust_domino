use arrayvec::ArrayVec;

use crate::{
    constants::BLOCK_STARTING_HAND_SIZE, direction::Direction, game_observer::GameObserver,
    player::Player, print_utils::tile_to_char_vertical, tile::Tile, tile_play::TilePlay, board::Board,
};

#[derive(Debug, Clone)]
pub struct HumanPlayer {
    hand: ArrayVec<Tile, BLOCK_STARTING_HAND_SIZE>,
}

impl HumanPlayer {
    pub fn new() -> Self {
        Self {
            hand: ArrayVec::new(),
        }
    }
}

impl Player for HumanPlayer {
    fn play_tile(
        &mut self,
        left: Option<u8>,
        right: Option<u8>,
        board: &Board,
    ) -> Option<TilePlay> {
        let mut playable_tiles = self
            .hand
            .iter()
            .enumerate()
            .flat_map(|(i, &t)| [(i, t), (i, t.flip())])
            .flat_map(|(i, t)| [(i, t, Direction::Left), (i, t, Direction::Right)])
            .filter(|(i, t, d)| match d {
                Direction::Left => left.map(|l| t.right == l).unwrap_or(true),
                Direction::Right => right.map(|r| t.left == r).unwrap_or(true),
            })
            .map(|(i, t, d)| {
                (
                    i,
                    TilePlay {
                        tile: t,
                        direction: d,
                    },
                )
            });

        let mut index = 0;
        for (i, tile_play) in playable_tiles.clone() {
            println!(
                "{}: {} {:?}",
                index,
                tile_to_char_vertical(tile_play.tile),
                tile_play.direction
            );
            index += 1;
        }

        let mut count = playable_tiles.clone().count();
        if count == 0 {
            println!("No playable tiles");
            return None;
        }
        let mut input = get_input(count);

        return playable_tiles.nth(input as usize).map(|(i, tile_play)| {
            self.hand.remove(i);
            tile_play
        });
    }

    fn draw_tiles(&mut self, starting_hand: ArrayVec<Tile, BLOCK_STARTING_HAND_SIZE>) {
        self.hand = starting_hand;
    }

    fn hand(&self) -> &[Tile] {
        &self.hand
    }

    fn box_clone(&self) -> Box<dyn Player> {
        Box::new(self.clone())
    }

    fn hand_sum(&self) -> u8 {
        self.hand.iter().map(|t| t.left + t.right).sum()
    }

    fn name(&self) -> &str {
        "Human Player"
    }
}

fn get_input(max: usize) -> usize {
    loop {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();

        if let Ok(n) = input.parse::<usize>() {
            if n < max {
                return n;
            }
        }
    }
}

impl GameObserver for HumanPlayer {}
