use arrayvec::ArrayVec;
use rand::{rngs::ThreadRng, seq::SliceRandom};

use crate::{board::Board, player::Player, tile::Tile, constants::BLOCK_STARTING_HAND_SIZE, print_utils::print_game, debug_utils::{is_board_valid, is_hand_playable}, tile_play::TilePlay};

pub enum GameState {
    Draw,
    Win(u8, u32),
    InProgress,
}

pub struct Game {
    board: Board,
    players: [Box<dyn Player>; 2],
    current_player: u8,
    blocked_count: u8,
}

impl Clone for Game {
    fn clone(&self) -> Self {
        Game {
            board: self.board.clone(),
            players: [self.players[0].box_clone(), self.players[1].box_clone()],
            current_player: self.current_player,
            blocked_count: self.blocked_count,
        }
    }
}

impl Game {
    pub fn new(players: [Box<dyn Player>; 2], rng: &mut ThreadRng) -> Game {
        let mut boneyard: ArrayVec<Tile, 28> = (0..7)
            .flat_map(|left| (left..7).map(move |right| Tile { left, right }))
            .collect();

        boneyard.shuffle(rng);

        let mut players = players;
        let mut starting_hands = ArrayVec::<ArrayVec<Tile, BLOCK_STARTING_HAND_SIZE>, 2>::new();
        starting_hands.push(ArrayVec::new());
        starting_hands.push(ArrayVec::new());
        let mut index = 0;
        for player in players.iter_mut() {
            for _ in 0..BLOCK_STARTING_HAND_SIZE {
                starting_hands[index].push(boneyard.pop().unwrap());
            }
            player.draw_tiles(starting_hands[index].clone());
            index += 1;
        }

        players[0].game_started(starting_hands.pop().unwrap());
        players[1].game_started(starting_hands.pop().unwrap());

        Game {
            board: Board {
                tiles: ArrayVec::new(),
            },
            players,
            current_player: 0,
            blocked_count: 0,
        }
    }

    pub fn play(&mut self) -> GameState {
        let current_player_index = self.current_player as usize;        
        let tile = self.players[current_player_index].play_tile(self.board.left(), self.board.right(), &self.board);
        self.play_tile(tile)
    }

    pub fn play_tile(&mut self, tile: Option<TilePlay>) -> GameState {
        let current_player_index = self.current_player as usize;
        let opponent_index = self.current_player as usize ^ 1;

        if let Some(tile_play) = tile {
            if self.players[current_player_index].hand().is_empty() {
                return GameState::Win(
                    self.current_player,
                    self.players[opponent_index].hand_sum() as u32,
                );
            }

            self.board.play(tile_play);
            self.players[opponent_index].opponent_played(tile_play.tile, &self.board);
            assert!(is_board_valid(&self.board));
            self.blocked_count = 0;
        } else {
            assert!(!is_hand_playable(
                self.players[current_player_index].hand(),
                self.board.left(),
                self.board.right(),
            ));

            self.players[opponent_index].opponent_was_blocked(&self.board);

            self.blocked_count += 1;
            if self.blocked_count == 2 {
                let opponent_score = self.players[opponent_index].hand_sum();
                let current_score = self.players[current_player_index].hand_sum();

                return match opponent_score.cmp(&current_score) {
                    std::cmp::Ordering::Less => GameState::Win(current_player_index as u8, opponent_score as u32),
                    std::cmp::Ordering::Greater => GameState::Win(opponent_index as u8, current_score as u32),
                    std::cmp::Ordering::Equal => GameState::Draw,
                };
            }
        }

        self.current_player ^= 1;
        GameState::InProgress
    }
}
