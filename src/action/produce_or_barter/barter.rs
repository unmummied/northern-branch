use super::StockInt;
use crate::card::{Card, Value, ValueInt};
use std::{collections::BTreeMap, iter};

#[derive(Debug, Clone)]
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
    GiveNTakeN {
        give: BTreeMap<Card, StockInt>,
        take: BTreeMap<Card, StockInt>,
    },
}

impl Barter {
    #[allow(clippy::cast_possible_wrap)]
    fn value_of_give(&self) -> ValueInt {
        match self {
            Self::Give1Take1 { give, take: _ } | Self::Give1TakeN { give, take: _ } => give.value(),
            Self::GiveNTake1 { give, take: _ } | Self::GiveNTakeN { give, take: _ } => give
                .iter()
                .map(|(card, n)| card.value() * (*n as ValueInt))
                .sum(),
        }
    }

    #[allow(clippy::cast_possible_wrap)]
    fn value_of_take(&self) -> ValueInt {
        match self {
            Self::Give1Take1 { give: _, take } | Self::GiveNTake1 { give: _, take } => take.value(),
            Self::Give1TakeN { give: _, take } | Self::GiveNTakeN { give: _, take } => take
                .iter()
                .map(|(card, n)| card.value() * (*n as ValueInt))
                .sum(),
        }
    }

    pub fn is_valid(&self) -> bool {
        self.value_of_take() <= self.value_of_give()
    }

    pub fn force_into_give_n_take_n(self) -> Self {
        match self {
            Self::Give1Take1 { give, take } => Self::GiveNTakeN {
                give: iter::once((give, 1)).collect(),
                take: iter::once((take, 1)).collect(),
            },
            Self::Give1TakeN { give, take } => Self::GiveNTakeN {
                give: iter::once((give, 1)).collect(),
                take,
            },
            Self::GiveNTake1 { give, take } => Self::GiveNTakeN {
                give,
                take: iter::once((take, 1)).collect(),
            },
            Self::GiveNTakeN { .. } => self,
        }
    }
}

#[allow(clippy::cast_possible_wrap)]
fn btree_map_value(map: &BTreeMap<Card, StockInt>) -> ValueInt {
    map.iter().fold(0, |mut acc, (card, n)| {
        acc += card.value() * *n as ValueInt;
        acc
    })
}
