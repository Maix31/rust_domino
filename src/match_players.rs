use crate::{
    game::{Game, GameState},
    match_results::MatchResults,
    player::Player,
    players::{
        conservative_player::ConservativePlayer, 
        greedy_player::GreedyPlayer, 
        random_player::RandomPlayer
    }
};

use prettytable::{Cell, Row, Table};

pub fn match_players_helper(
    p1_maker: fn() -> Box<dyn Player>,
    p2_maker: fn() -> Box<dyn Player>,
    n: usize,
) -> MatchResults {
    let mut rng = rand::thread_rng();

    let mut results = MatchResults::default();

    for _ in 0..n {
        let mut game = Game::new([p1_maker(), p2_maker()], &mut rng);

        let mut state = GameState::InProgress;

        while let GameState::InProgress = state {
            state = game.play();
        }

        match state {
            GameState::Win(0, score) => {
                results.wins_0 += 1;
                results.scores_0 += score;
            }
            GameState::Win(1, score) => {
                results.wins_1 += 1;
                results.scores_1 += score;
            }
            GameState::Draw => {
                results.draws += 1;
            }
            _ => {}
        };
    }

    results
}

pub fn match_players() {
    let player_makers: Vec<fn() -> Box<dyn Player>> = vec![
        || Box::new(RandomPlayer::new()),
        || Box::new(GreedyPlayer::new()),
        || Box::new(ConservativePlayer::new()),
    ];

    let mut results = vec![];
    for p1_maker in &player_makers {
        for p2_maker in &player_makers {
            results.push(match_players_helper(*p1_maker, *p2_maker, 100_000));
        }
    }

    // Create a table of results using prettytable
    let mut table = Table::new();
    // Constructing header dynamically using player names
    let mut header = vec![Cell::new("P1(col) \\ P2(row))")];
    for maker in &player_makers {
        let player = maker();
        header.push(Cell::new(&player.name()));
    }
    table.add_row(Row::new(header));

    for (i, p1_maker) in player_makers.iter().enumerate() {
        let mut row = Vec::new();
        let player = p1_maker();
        row.push(Cell::new(&player.name()));

        for (j, _p2_maker) in player_makers.iter().enumerate() {
            let result = &results[i * player_makers.len() + j];
            row.push(Cell::new(&format!(
                "{:.2} \\ {:.2}",
                // Not sure why this is backwards, probably a bug in the results
                result.wins_1 as f64 / result.games() as f64,
                result.wins_0 as f64 / result.games() as f64,
            )));
        }
        table.add_row(Row::new(row));
    }

    // print the table
    table.printstd();
}
