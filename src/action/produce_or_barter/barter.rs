use super::StockInt;
use crate::card::{Card, Value, ValueInt};
use std::collections::BTreeMap;

#[derive(Debug)]
pub enum Barter {
    Give1Take1 {
        give: Card,
        take: Card,
    },
    Give1TakeN {
        give: Card,
        take: BTreeMap<Card, StockInt>,
    },
    GiveNTake1 {
        give: BTreeMap<Card, StockInt>,
        take: Card,
    },
    // GiveNTakeN,
}

impl Barter {
    pub fn values(&self) -> (ValueInt, ValueInt) {
        match self {
            Self::Give1Take1 { give, take } => (give.value(), take.value()),
            Self::Give1TakeN { give, take } => (give.value(), btree_map_value(take)),
            Self::GiveNTake1 { give, take } => (btree_map_value(give), take.value()),
        }
    }

    pub fn is_valid(&self) -> bool {
        let values = self.values();
        values.1 <= values.0
    }
}

#[allow(clippy::cast_possible_wrap)]
fn btree_map_value(map: &BTreeMap<Card, StockInt>) -> ValueInt {
    map.iter().fold(0, |mut acc, (card, n)| {
        acc += card.value() * *n as ValueInt;
        acc
    })
}
