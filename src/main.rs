use core::num;
use std::sync::{atomic::AtomicI64, Arc};

use arrayvec::ArrayVec;
use rand::{rngs::ThreadRng, seq::SliceRandom};

const BLOCK_STARTING_HAND_SIZE: usize = 7;
const MAX_BOARD_SIZE: usize = 13;

#[derive(Debug, Clone, Copy)]
struct Tile {
    left: u8,
    right: u8,
}

impl Tile {
    fn flip(self) -> Tile {
        Tile {
            left: self.right,
            right: self.left,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

struct TilePlay {
    tile: Tile,
    direction: Direction,
}

struct Board {
    tiles: ArrayVec<Tile, MAX_BOARD_SIZE>,
}

impl Board {
    fn is_empty(&self) -> bool {
        self.tiles.is_empty()
    }

    fn left(&self) -> Option<u8> {
        self.tiles.first().map(|t| t.left)
    }

    fn right(&self) -> Option<u8> {
        self.tiles.last().map(|t| t.right)
    }

    fn play(&mut self, tile_play: TilePlay) {
        if self.is_empty() {
            self.tiles.push(tile_play.tile);
            return;
        }

        match tile_play.direction {
            Direction::Left => {
                self.tiles.insert(0, tile_play.tile);
            }
            Direction::Right => {
                self.tiles.push(tile_play.tile);
            }
        };
    }
}

trait GameObserver {
    fn game_started(&mut self);
    fn opponent_played(&mut self, tile: Tile);
    fn opponent_drew(&mut self);
    fn opponent_was_blocked(&mut self);
}

trait Player: GameObserver {
    fn play_tile(&mut self, left: Option<u8>, right: Option<u8>) -> Option<TilePlay>;
    fn draw_tiles(&mut self, starting_hand: ArrayVec<Tile, BLOCK_STARTING_HAND_SIZE>);
    fn hand(&self) -> &[Tile];
}

#[derive(Debug, Clone)]
struct HumanPlayer {
    hand: ArrayVec<Tile, BLOCK_STARTING_HAND_SIZE>,
}

struct RandomPlayer {
    hand: ArrayVec<Tile, BLOCK_STARTING_HAND_SIZE>,
}

impl Player for RandomPlayer {
    fn play_tile(&mut self, left: Option<u8>, right: Option<u8>) -> Option<TilePlay> {
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

        playable_tiles.next().map(|(i, tile_play)| {
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
}

impl Player for HumanPlayer {
    fn play_tile(&mut self, left: Option<u8>, right: Option<u8>) -> Option<TilePlay> {
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

        let index = 0;
        for (i, tile_play) in playable_tiles.clone() {
            println!(
                "{}: {} {:?}",
                index,
                tile_to_char_vertical(tile_play.tile),
                tile_play.direction
            );
        }

        playable_tiles.next().map(|(i, tile_play)| {
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
}

impl GameObserver for HumanPlayer {
    fn game_started(&mut self) {}
    fn opponent_played(&mut self, tile: Tile) {}
    fn opponent_drew(&mut self) {}
    fn opponent_was_blocked(&mut self) {}
}

impl GameObserver for RandomPlayer {
    fn game_started(&mut self) {}
    fn opponent_played(&mut self, tile: Tile) {}
    fn opponent_drew(&mut self) {}
    fn opponent_was_blocked(&mut self) {}
}

enum GameState {
    Draw,
    Win(u8),
}

struct Game {
    board: Board,
    players: [RandomPlayer; 2],
    current_player: u8,
    blocked_count: u8,
}

impl Game {
    fn new(players: [RandomPlayer; 2], rng: &mut ThreadRng) -> Game {
        let mut boneyard: ArrayVec<Tile, 28> = (0..7)
            .flat_map(|left| (left..7).map(move |right| Tile { left, right }))
            .collect();

        boneyard.shuffle(rng);

        let mut players = players;

        for player in players.iter_mut() {
            let mut starting_hand = ArrayVec::new();
            for _ in 0..BLOCK_STARTING_HAND_SIZE {
                starting_hand.push(boneyard.pop().unwrap());
            }
            player.draw_tiles(starting_hand);
        }

        Game {
            board: Board {
                tiles: ArrayVec::new(),
            },
            players,
            current_player: 0,
            blocked_count: 0,
        }
    }

    fn play(&mut self) -> GameState {
        loop {
            // print_game(&self.board, &self.players);

            let current_player_index = self.current_player as usize;
            // let opponent_index = self.current_player as usize ^ 1;

            if let Some(tile_play) =
                self.players[current_player_index].play_tile(self.board.left(), self.board.right())
            {
                if self.players[current_player_index].hand().is_empty() {
                    return GameState::Win(self.current_player);
                }

                self.board.play(tile_play);
                self.blocked_count = 0;
                self.current_player ^= 1;
            } else {
                if self.blocked_count == 2 {
                    return GameState::Draw;
                }
                self.blocked_count += 1;
            }
        }
    }
}

fn tile_to_char_vertical(tile: Tile) -> char {
    let tiles = [
        ['🁣', '🁤', '🁥', '🁦', '🁧', '🁨', '🁩'],
        ['🁪', '🁫', '🁬', '🁭', '🁮', '🁯', '🁰'],
        ['🁱', '🁲', '🁳', '🁴', '🁵', '🁶', '🁷'],
        ['🁸', '🁹', '🁺', '🁻', '🁼', '🁽', '🁾'],
        ['🁿', '🂀', '🂁', '🂂', '🂃', '🂄', '🂅'],
        ['🂆', '🂇', '🂈', '🂉', '🂊', '🂋', '🂌'],
        ['🂍', '🂎', '🂏', '🂐', '🂑', '🂒', '🂓'],
    ];

    tiles[tile.left as usize][tile.right as usize]
}

fn tile_to_char_horizontal(tile: Tile) -> char {
    let tiles = [
        ['🀱', '🀲', '🀳', '🀴', '🀵', '🀶', '🀷'],
        ['🀸', '🀹', '🀺', '🀻', '🀼', '🀽', '🀾'],
        ['🀿', '🁀', '🁁', '🁂', '🁃', '🁄', '🁅'],
        ['🁆', '🁇', '🁈', '🁉', '🁊', '🁋', '🁌'],
        ['🁍', '🁎', '🁏', '🁐', '🁑', '🁒', '🁓'],
        ['🁔', '🁕', '🁖', '🁗', '🁘', '🁙', '🁚'],
        ['🁛', '🁜', '🁝', '🁞', '🁟', '🁠', '🁡'],
    ];

    tiles[tile.left as usize][tile.right as usize]
}

fn print_game(board: &Board, player: &[Box<dyn Player>; 2]) {
    let mut player_string = String::new();

    for tile in player[0].hand() {
        player_string.push(tile_to_char_vertical(*tile));
        player_string.push(' ');
    }

    println!("{}", player_string);

    let mut board_string = String::new();

    for tile in board.tiles.iter() {
        board_string.push(tile_to_char_horizontal(*tile));
        board_string.push(' ');
    }

    println!("{}", board_string);

    let mut opponent_string = String::new();

    for tile in player[1].hand() {
        opponent_string.push(tile_to_char_vertical(*tile));
        opponent_string.push(' ');
    }

    println!("{}", opponent_string);
    println!("---------------------------------------");
}

fn benchmarck<F>(f: F)
where
    F: Fn(),
{
    let now = std::time::Instant::now();
    f();
    println!("{}ms", now.elapsed().as_millis());
}

fn benchmark_n<F>(n: usize, f: F)
where
    F: Fn(),
{
    let now = std::time::Instant::now();
    for _ in 0..n {
        f();
    }
    println!("{}ms", now.elapsed().as_millis());
}

fn main() {
    let wins_0 = Arc::new(AtomicI64::new(0));
    let wins_1 = Arc::new(AtomicI64::new(0));
    let draws  = Arc::new(AtomicI64::new(0));
    let games  = Arc::new(AtomicI64::new(0));

    let num_cpus = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1);

    for _ in 0..num_cpus {
        let wins_0 = wins_0.clone();
        let wins_1 = wins_1.clone();
        let draws  = draws.clone();
        let games  = games.clone();

            std::thread::spawn(move || {
                let mut rng = rand::thread_rng();

                loop {
                    let mut game = Game::new(
                        [
                            RandomPlayer {
                                hand: ArrayVec::new(),
                            },
                            RandomPlayer {
                                hand: ArrayVec::new(),
                            },
                        ],
                        &mut rng,
                    );

                    let state = game.play();

                    match state {
                        GameState::Win(0) => {
                            wins_0.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        }
                        GameState::Win(1) => {
                            wins_1.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        }
                        GameState::Draw => {
                            draws.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        }
                        _ => {}
                    };

                    games.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                }
            });
        }
        let now = std::time::Instant::now();

        loop {
            std::thread::sleep(std::time::Duration::from_secs(5));
            let count = games.load(std::sync::atomic::Ordering::Relaxed);
            println!("{} games played", count);
            println!("{} games per second", count as u64 / now.elapsed().as_secs());
            let w0 = wins_0.load(std::sync::atomic::Ordering::Relaxed);
            let w1 = wins_1.load(std::sync::atomic::Ordering::Relaxed);
            println!("{} games won by player 0", w0);
            println!("{} games won by player 1", w1);
            // println!("allocated bytes before main: {}", ALLOCATED.load(Relaxed));
        }
}
