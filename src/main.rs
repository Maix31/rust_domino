use std::sync::{atomic::AtomicI32, Arc};

use first_possible_tile_ai_player::FirstPossibleTileAIPlayer;
use game::{GameState, Winner};
use greedy_ai_player::GreedyAIPlayer;
use human_player::HumanPlayer;

use crate::game::Game;

mod boneyard;
mod game;
mod hand;
mod snake;
mod tile;
mod human_player;
mod choose_tile_strategy;
mod player;
mod game_observer;
mod greedy_ai_player;
mod first_possible_tile_ai_player;

use std::thread::available_parallelism;

// use std::alloc::{System, GlobalAlloc, Layout};
// use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

// struct Counter;

// static ALLOCATED: AtomicUsize = AtomicUsize::new(0);

// unsafe impl GlobalAlloc for Counter {
//     unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
//         let ret = System.alloc(layout);
//         if !ret.is_null() {
//             ALLOCATED.fetch_add(layout.size(), Relaxed);
//         }
//         ret
//     }

//     unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
//         System.dealloc(ptr, layout);
//         ALLOCATED.fetch_sub(layout.size(), Relaxed);
//     }
// }

// #[global_allocator]
// static A: Counter = Counter;

fn stress_test() {
    // create an atomic counter
    let mut counter = Arc::new(AtomicI32::new(0));
    let mut winner_0 = Arc::new(AtomicI32::new(0));
    let mut winner_1 = Arc::new(AtomicI32::new(0));
    let mut _score_0 = Arc::new(AtomicI32::new(0));
    let mut _score_1 = Arc::new(AtomicI32::new(0));
    let default_parallelism_approx = available_parallelism().unwrap().get();
    // let default_parallelism_approx = 1;

    let mut threads = Vec::new();
    for _ in 0..default_parallelism_approx {
        let counter = counter.clone();
        let winner_0 = winner_0.clone();
        let winner_1 = winner_1.clone();
        let _score_0 = _score_0.clone();
        let _score_1 = _score_1.clone();

        threads.push(std::thread::spawn(move || {
            let mut should_swap = false;
            loop {
                let mut game = Game::<GreedyAIPlayer, FirstPossibleTileAIPlayer>::new().swap_players(should_swap);
                loop {
                    let (new_game, state) = game.play();
                    game = new_game;
                    if let GameState::Finished { winner , score_0, score_1  } = state {
                        if let Some(winner) = winner {
                           match winner {
                               Winner::Player0 => winner_0.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
                               Winner::Player1 => winner_1.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
                           };
                            _score_0.fetch_add(score_0, std::sync::atomic::Ordering::Relaxed);
                            _score_1.fetch_add(score_1, std::sync::atomic::Ordering::Relaxed);       
                        } else {

                        }

                        break;
                    }
                    
                }
                should_swap = !should_swap;
                counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }
        }));
    }

    let now = std::time::Instant::now();

    loop {
        std::thread::sleep(std::time::Duration::from_secs(5));
        let count = counter.load(std::sync::atomic::Ordering::Relaxed);
        println!("{} games played", count);
        println!("{} games per second", count as u64 / now.elapsed().as_secs());
        let w0 = winner_0.load(std::sync::atomic::Ordering::Relaxed);
        let w1 = winner_1.load(std::sync::atomic::Ordering::Relaxed);
        let s0 = _score_0.load(std::sync::atomic::Ordering::Relaxed);
        let s1 = _score_1.load(std::sync::atomic::Ordering::Relaxed);
        println!("{} games won by player 0", w0);
        println!("{} games won by player 1", w1);
        println!("{} average score for player 0", s0.checked_div(w0).unwrap_or_default());
        println!("{} average score for player 1", s1.checked_div(w1).unwrap_or_default());
        // println!("allocated bytes before main: {}", ALLOCATED.load(Relaxed));
    }
}

fn singleplayer() {
    // make the game infinite
    loop {
        let mut game = Game::<HumanPlayer, HumanPlayer>::new();
        // the actual game loop
        loop {
            println!("{:#?}", game);
            let (new_game, state) = game.play();
            game = new_game;
            if let GameState::Finished { winner , score_0, score_1  } = state {
                if let Some(winner) = winner {
                    println!("Player {:?} won with score {}", winner, score_0);
                } else {
                    println!("Draw with score {} - {}", score_0, score_1);
                }                
                break;
            }
        }
    }
}

fn main() {
    // singleplayer();
    stress_test();
}
