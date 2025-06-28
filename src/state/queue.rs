pub mod player;

use player::Name;
use std::collections::VecDeque;

type QueueInt = u8;

#[derive(Debug)]
pub struct Queue {
    queue: VecDeque<Name>,
}

impl Queue {
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn curr_turn(&self) -> Option<Name> {
        self.queue.front().copied()
    }

    pub fn rotate_turn(&mut self) {
        if let Some(curr) = self.queue.pop_front() {
            self.queue.push_back(curr);
        }
    }
}
