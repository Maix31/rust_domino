use crate::{board::Board, tile::Tile};

pub fn is_hand_playable(hand: &[Tile], left: Option<u8>, right: Option<u8>) -> bool {
    hand.iter().any(|t| {
        left.map(|l| t.left == l || t.right == l).unwrap_or(true)
            || right.map(|r| t.right == r || t.left == r).unwrap_or(true)
    })
}

pub fn is_board_valid(board: &Board) -> bool {
    if board.tiles.is_empty() {
        return true;
    }

    let mut left = board.tiles.first().unwrap().left;
    let mut right = board.tiles.first().unwrap().right;

    for tile in board.tiles.iter().skip(1) {
        if tile.left == left || tile.right == left {
            left = tile.left;
            right = tile.right;
        } else if tile.left == right || tile.right == right {
            left = tile.right;
            right = tile.left;
        } else {
            return false;
        }
    }

    true
}
