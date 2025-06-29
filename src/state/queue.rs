use super::PopulationInt;
use std::collections::VecDeque;

pub const MINIMUM_PLAYERS_LEN: PopulationInt = 2;
pub const MAXIMUM_PLAYERS_LEN: PopulationInt = 4;
pub const TOO_FEW_PLAYERS_ERR: &str = "too few players...";
pub const TOO_MUCH_PLAYERS_ERR: &str = "too mush players...";

type QueueInt = u8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Name {
    Alice,
    Bob,
    Charlie,
    David,
}

#[derive(Debug)]
pub struct Queue {
    queue: VecDeque<Name>,
}

impl Queue {
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn peek(&self) -> Option<Name> {
        self.queue.front().copied()
    }

    pub fn rotate_turn(&mut self) {
        if let Some(curr) = self.queue.pop_front() {
            self.queue.push_back(curr);
        }
    }
}

impl TryFrom<PopulationInt> for Queue {
    type Error = &'static str;

    fn try_from(population: PopulationInt) -> Result<Self, Self::Error> {
        use Name::{Alice, Bob, Charlie, David};
        match population {
            0 | 1 => Err(TOO_FEW_PLAYERS_ERR),
            2 => Ok(Self {
                queue: VecDeque::from([Alice, Bob]),
            }),
            3 => Ok(Self {
                queue: VecDeque::from([Alice, Bob, Charlie]),
            }),
            4 => Ok(Self {
                queue: VecDeque::from([Alice, Bob, Charlie, David]),
            }),
            _ => Err(TOO_MUCH_PLAYERS_ERR),
        }
    }
}
