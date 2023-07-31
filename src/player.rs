use crate::{hand::HasHandTrait, choose_tile_strategy::ChooseTileStrategy, game_observer::GameObserver};

pub trait Player: HasHandTrait + ChooseTileStrategy + GameObserver + Default {

}
