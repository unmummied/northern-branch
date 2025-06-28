pub mod board;
pub mod queue;

use board::BoardState;
use queue::Queue;

pub type PopulationInt = usize;

#[derive(Debug)]
pub struct GameState {
    queue: Queue,
    board: BoardState,
}

impl GameState {
    pub fn from_population() -> Self {
        todo!()
    }
}
