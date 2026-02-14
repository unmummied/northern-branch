use super::PopulationInt;
use std::{
    collections::VecDeque,
    fmt::{self, Display, Formatter},
};

pub const MINIMUM_PLAYERS_LEN: PopulationInt = 2;
pub const MAXIMUM_PLAYERS_LEN: PopulationInt = 4;
pub const ERR_TOO_FEW_PLAYERS: &str = "too few players...";
pub const ERR_TOO_MUCH_PLAYERS: &str = "too mush players...";

type QueueInt = u8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Name {
    Alice,
    Bob,
    Charlie,
    David,
}

#[derive(Debug, Clone)]
pub struct Queue {
    queue: VecDeque<Name>,
}

impl Queue {
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn curr_player(&self) -> Option<Name> {
        self.queue.front().copied()
    }

    pub fn rotate_turn(&mut self) {
        if let Some(curr) = self.queue.pop_front() {
            self.queue.push_back(curr);
        }
    }

    pub fn members(&self) -> impl Iterator<Item = Name> {
        self.queue.iter().copied()
    }
}

impl TryFrom<PopulationInt> for Queue {
    type Error = &'static str;

    fn try_from(population: PopulationInt) -> Result<Self, Self::Error> {
        use Name::{Alice, Bob, Charlie, David};
        match population {
            0 | 1 => Err(ERR_TOO_FEW_PLAYERS),
            2 => Ok(Self {
                queue: [Alice, Bob].into_iter().collect(),
            }),
            3 => Ok(Self {
                queue: [Alice, Bob, Charlie].into_iter().collect(),
            }),
            4 => Ok(Self {
                queue: [Alice, Bob, Charlie, David].into_iter().collect(),
            }),
            _ => Err(ERR_TOO_MUCH_PLAYERS),
        }
    }
}

impl Display for Queue {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self.queue)
    }
}
