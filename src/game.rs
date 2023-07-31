use std::{
    env::current_exe,
    fmt::{write, Debug},
    thread::current,
};

use crate::{
    boneyard::Boneyard,
    hand::{Hand, HandTrait},
    player::Player,
    snake::Snake,
    tile::Tile,
};

pub struct Game<P1: Player, P2: Player> {
    blocked_counter: i32,
    current_player: i32,
    players: (P1, P2),
    snake: Snake,
    boneyard: Boneyard,
    game_mode: GameMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Winner {
    Player0,
    Player1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameMode {
    Draw,
    Block,
}

pub enum GameState {
    Playing,
    Finished {
        winner: Option<Winner>,
        score_0: i32,
        score_1: i32,
    },
}

impl<P1: Player, P2: Player> Game<P1, P2> {
    pub fn new() -> Game<P1, P2> {
        let mut boneyard = Boneyard::new().shuffle();

        let mut player_0: P1 = Default::default();
        let mut player_1: P2 = Default::default();

        player_0.hand_mut().add_multiple(boneyard.draw_n(7));
        player_1.hand_mut().add_multiple(boneyard.draw_n(7));

        player_0.game_started();
        player_1.game_started();
        
        Game {
            blocked_counter: 0,
            current_player: 0,
            players: (player_0, player_1),
            snake: Snake::new(),
            boneyard,
            game_mode: GameMode::Draw,
        }
    }

    pub fn play(mut self) -> (Self, GameState) {
        if self.blocked_counter == 2 {
            return self.game_finished();
        }

        if self.current_player == 0 {
            let current_player = &mut self.players.0;

            let is_boneyard_empty = self.game_mode == GameMode::Draw && self.boneyard.is_empty();
            let is_hand_playable = Self::is_hand_playable(&current_player.hand(), &self.snake);
    
            match (!is_hand_playable, is_boneyard_empty) {
                (true, true) => {
                    self.blocked_counter += 1;
                    return (self, GameState::Playing);
                }
                (true, false) => {
                    while !self.boneyard.is_empty() {
                        let tile = self.boneyard.draw();
                        current_player.hand_mut().add(tile);
                        if self.snake.is_playable(tile) {
                            break;
                        }
                    }
    
                    if !Self::is_hand_playable(&current_player.hand(), &self.snake) {
                        self.blocked_counter += 1;
                        return (self, GameState::Playing);
                    }
                }
                (false, _) => {}
            };
    
            assert!(Self::is_hand_playable(&current_player.hand(), &self.snake));
    
            let tile = current_player.choose_tile(&self.snake);
            self.snake.add(tile);
    
            if current_player.hand().is_empty() {
                return self.game_finished();
            }
    
            self.current_player = (self.current_player + 1) % 2;
            (self, GameState::Playing)

        } else {
            let current_player = &mut self.players.1;
            let opponent = &mut self.players.0;

            let is_boneyard_empty = self.game_mode == GameMode::Draw && self.boneyard.is_empty();
            let is_hand_playable = Self::is_hand_playable(&current_player.hand(), &self.snake);
    
            match (!is_hand_playable, is_boneyard_empty) {
                (true, true) => {
                    self.blocked_counter += 1;

                    // player cannot be blocked if there aren't any tiles in the snake
                    assert!(!self.snake.is_empty());
                    opponent.opponent_was_blocked([self.snake.left().unwrap(), self.snake.right().unwrap()]);
                    return (self, GameState::Playing);
                }
                (true, false) => {
                    while !self.boneyard.is_empty() {
                        let tile = self.boneyard.draw();
                        current_player.hand_mut().add(tile);
                        opponent.opponent_drew_tile();
                        if self.snake.is_playable(tile) {
                            break;
                        }
                    }
    
                    if !Self::is_hand_playable(&current_player.hand(), &self.snake) {
                        self.blocked_counter += 1;
                        // player cannot be blocked if there aren't any tiles in the snake
                        assert!(!self.snake.is_empty());
                        opponent.opponent_was_blocked([self.snake.left().unwrap(), self.snake.right().unwrap()]);
                        return (self, GameState::Playing);
                    }
                }
                (false, _) => {}
            };
    
            assert!(Self::is_hand_playable(&current_player.hand(), &self.snake));
    
            let tile = current_player.choose_tile(&self.snake);
            self.snake.add(tile);

            opponent.opponent_played_tile(tile);
    
            if current_player.hand().is_empty() {
                return self.game_finished();
            }
    
            self.current_player = (self.current_player + 1) % 2;
            (self, GameState::Playing)
        }
    }

    fn game_finished(self) -> (Self, GameState) {
        let score_0 = self.players.0.hand().score();
        let score_1 = self.players.1.hand().score();

        let winner = match score_0.cmp(&score_1) {
            std::cmp::Ordering::Less => Some(Winner::Player0),
            std::cmp::Ordering::Greater => Some(Winner::Player1),
            std::cmp::Ordering::Equal => None,
        };

        // the winner gets the score of the opponent added to their score
        let score_0 = match winner {
            Some(Winner::Player0) => score_0 + score_1,
            _ => score_0,
        };

        let score_1 = match winner {
            Some(Winner::Player1) => score_1 + score_0,
            _ => score_1,
        };

        (
            self,
            GameState::Finished {
                winner,
                score_0,
                score_1,
            },
        )
    }

    fn is_hand_playable(hand: &Hand, snake: &Snake) -> bool {
        hand.tiles.iter().any(|tile| snake.is_playable(*tile))
    }

    pub fn swap_players(mut self, should_swap: bool) -> Self {
        if should_swap {
            self.current_player = (self.current_player + 1) % 2;
        }
        self
    }
}

impl<P1: Player, P2: Player> Debug for Game<P1, P2> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tiles_unicode_snake = [
            ['🀱', '🀲', '🀳', '🀴', '🀵', '🀶', '🀷'],
            ['🀸', '🀹', '🀺', '🀻', '🀼', '🀽', '🀾'],
            ['🀿', '🁀', '🁁', '🁂', '🁃', '🁄', '🁅'],
            ['🁆', '🁇', '🁈', '🁉', '🁊', '🁋', '🁌'],
            ['🁍', '🁎', '🁏', '🁐', '🁑', '🁒', '🁓'],
            ['🁔', '🁕', '🁖', '🁗', '🁘', '🁙', '🁚'],
            ['🁛', '🁜', '🁝', '🁞', '🁟', '🁠', '🁡'],
        ];

        let tiles_unicode_hand = [
            ['🁣', '🁤', '🁥', '🁦', '🁧', '🁨', '🁩'],
            ['🁪', '🁫', '🁬', '🁭', '🁮', '🁯', '🁰'],
            ['🁱', '🁲', '🁳', '🁴', '🁵', '🁶', '🁷'],
            ['🁸', '🁹', '🁺', '🁻', '🁼', '🁽', '🁾'],
            ['🁿', '🂀', '🂁', '🂂', '🂃', '🂄', '🂅'],
            ['🂆', '🂇', '🂈', '🂉', '🂊', '🂋', '🂌'],
            ['🂍', '🂎', '🂏', '🂐', '🂑', '🂒', '🂓'],
        ];

        let print_hand = |hand: &Hand| {
            let mut result = String::new();
            for tile in hand.tiles.iter() {
                result.push_str(&format!(
                    " {}",
                    tiles_unicode_hand[tile.left as usize][tile.right as usize]
                ));
            }
            result
        };

        let print_snake = |snake: &Snake| {
            let mut result = String::new();
            for tile in snake.tiles.iter() {
                result.push_str(&format!(
                    " {}",
                    tiles_unicode_snake[tile.left as usize][tile.right as usize]
                ));
            }
            result
        };

        write!(
            f,
            "Hand 0: {}\nSnake {}\nHand 1: {}",
            print_hand(&self.players.0.hand()),
            print_snake(&self.snake),
            print_hand(&self.players.1.hand())
        )
    }
}
