pub mod board;
pub mod player;
pub mod queue;

use board::BoardState;
use player::Inventory;
use queue::Queue;
use rand::Rng;
use std::collections::BTreeSet;

pub type PopulationInt = usize;

#[derive(Debug)]
pub struct GameState {
    players: BTreeSet<Inventory>,
    queue: Queue,
    board: BoardState,
}

impl GameState {
    pub fn begin<R: Rng>(_rng: &mut R, _population: PopulationInt) -> Self {
        todo!()
    }
}
