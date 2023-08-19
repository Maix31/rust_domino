use arrayvec::ArrayVec;

use crate::{
    board::Board, constants::BLOCK_STARTING_HAND_SIZE, direction::Direction,
    game_observer::GameObserver, player::Player, tile::Tile, tile_play::TilePlay,
};

#[derive(Debug, Clone)]
pub struct PerfectObserverTryBlockOpponent {
    hand: ArrayVec<Tile, BLOCK_STARTING_HAND_SIZE>,
    opponent_hand: ArrayVec<Tile, BLOCK_STARTING_HAND_SIZE>,
}

impl PerfectObserverTryBlockOpponent {
    pub fn new() -> Self {
        Self {
            hand: ArrayVec::new(),
            opponent_hand: ArrayVec::new(),
        }
    }
}

impl Player for PerfectObserverTryBlockOpponent {
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
            .min_by_key(|(_, tile_play)| tile_play.tile.left + tile_play.tile.right)
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
        "Conservative Player"
    }
}

impl GameObserver for PerfectObserverTryBlockOpponent {
    fn game_started(&mut self, opponent_tiles: ArrayVec<Tile, BLOCK_STARTING_HAND_SIZE>) {
        self.opponent_hand = opponent_tiles;
    }
    fn opponent_played(&mut self, tile: Tile, board: &Board) {
        self.opponent_hand.retain(|t| *t != tile);
    }
    fn opponent_drew(&mut self, tile: Tile) {
        self.opponent_hand.push(tile);
    }
    fn opponent_was_blocked(&mut self, board: &Board) {}
}
