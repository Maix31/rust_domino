use crate::{tile::Tile, player::Player, board::Board};

pub fn tile_to_char_vertical(tile: Tile) -> char {
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

pub fn tile_to_char_horizontal(tile: Tile) -> char {
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

pub fn print_game(board: &Board, player: &[Box<dyn Player>; 2]) {
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
