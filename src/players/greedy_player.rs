use arrayvec::ArrayVec;

use crate::{
    constants::BLOCK_STARTING_HAND_SIZE, direction::Direction, game_observer::GameObserver,
    player::Player, tile::Tile, tile_play::TilePlay, board::Board,
};

#[derive(Debug, Clone)]
pub struct GreedyPlayer {
    hand: ArrayVec<Tile, BLOCK_STARTING_HAND_SIZE>,
}

impl GreedyPlayer {
    pub fn new() -> Self {
        Self {
            hand: ArrayVec::new(),
        }
    }
}

impl Player for GreedyPlayer {
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

        playable_tiles
            .max_by_key(|(_, tile_play)| tile_play.tile.left + tile_play.tile.right)
            .map(|(i, tile_play)| {
                self.hand.remove(i);
                tile_play
            })
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
        "Greedy Player"
    }
}

impl GameObserver for GreedyPlayer {}
