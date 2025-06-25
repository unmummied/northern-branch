pub mod board;
pub mod queue;

use board::BoardState;
use queue::Queue;

#[derive(Debug)]
pub struct GameState {
    queue: Queue,
    board: BoardState,
}

impl GameState {}
