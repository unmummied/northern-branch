use crate::{action::produce_or_barter::StockInt, card::Card};
use std::{cmp::Ordering, collections::BTreeMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Name {
    Player1,
    Player2,
    Player3,
    Player4,
}

#[derive(Debug, Eq)]
pub struct Player {
    name: Name,
    has: BTreeMap<Card, StockInt>,
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

impl Ord for Player {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for Player {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
