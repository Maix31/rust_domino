use crate::benchmark::benchmark;
use crate::match_players::match_players;

mod game;
mod tile;
mod board;
mod direction;
mod tile_play;
mod match_results;

mod game_observer;
mod player;

mod players;

mod match_players;

mod constants;

mod benchmark;

mod debug_utils;
mod print_utils;

fn main() {
    benchmark( || {
        match_players();
    });
}
