use std::sync::{atomic::AtomicI64, Arc};

use crate::{game::{Game, GameState}};
use crate::players::{human_player::HumanPlayer, random_player::RandomPlayer};

pub fn stress_test() {
    let wins_0 = Arc::new(AtomicI64::new(0));
    let scores_0 = Arc::new(AtomicI64::new(0));
    let wins_1 = Arc::new(AtomicI64::new(0));
    let scores_1 = Arc::new(AtomicI64::new(0));
    let draws = Arc::new(AtomicI64::new(0));
    let games = Arc::new(AtomicI64::new(0));

    let num_cpus = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);

    for _ in 0..1 {
        // for _ in 0..num_cpus {
        let wins_0 = wins_0.clone();
        let scores_0 = scores_0.clone();
        let wins_1 = wins_1.clone();
        let scores_1 = scores_1.clone();
        let draws = draws.clone();
        let games = games.clone();

        std::thread::spawn(move || {
            let mut rng = rand::thread_rng();

            loop {
                let mut game = Game::new(
                    [Box::new(HumanPlayer::new()), Box::new(RandomPlayer::new())],
                    &mut rng,
                );

                let mut state = GameState::InProgress;

                while let GameState::InProgress = state {
                    state = game.play();
                }

                match state {
                    GameState::Win(0, score) => {
                        wins_0.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        scores_0.fetch_add(score as i64, std::sync::atomic::Ordering::Relaxed);
                    }
                    GameState::Win(1, score) => {
                        wins_1.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        scores_1.fetch_add(score as i64, std::sync::atomic::Ordering::Relaxed);
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
        println!(
            "{} games per second",
            count as u64 / now.elapsed().as_secs()
        );
        let w0 = wins_0.load(std::sync::atomic::Ordering::Relaxed);
        let w1 = wins_1.load(std::sync::atomic::Ordering::Relaxed);
        let d = draws.load(std::sync::atomic::Ordering::Relaxed);
        println!("{} games won by player 0", w0);
        println!("{} games won by player 1", w1);
        println!("{} games drawn", d);
    }
}


pub fn benchmark<F>(f: F)
where
    F: Fn(),
{
    let now = std::time::Instant::now();
    f();
    println!("{}ms", now.elapsed().as_millis());
}

pub fn benchmark_n<F>(n: usize, f: F)
where
    F: Fn(),
{
    let now = std::time::Instant::now();
    for _ in 0..n {
        f();
    }
    println!("{}ms", now.elapsed().as_millis());
}
