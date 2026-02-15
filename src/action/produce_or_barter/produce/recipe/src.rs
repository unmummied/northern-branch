use super::super::super::StockInt;
use crate::card::Card;
use std::{
    collections::BTreeMap,
    ops::{Add, AddAssign},
};

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Src {
    pub src: BTreeMap<Card, Usage>,
}

impl Src {
    pub fn consume_cards(self) -> BTreeMap<Card, StockInt> {
        self.src
            .into_iter()
            .map(
                |(
                    card,
                    Usage {
                        consumed,
                        retained: _,
                    },
                )| (card, consumed),
            )
            .collect()
    }
    pub fn retain_cards(self) -> BTreeMap<Card, StockInt> {
        self.src
            .into_iter()
            .map(
                |(
                    card,
                    Usage {
                        consumed: _,
                        retained,
                    },
                )| (card, retained),
            )
            .collect()
    }
}

impl<T, I> From<I> for Src
where
    I: IntoIterator<Item = (T, (StockInt, StockInt))>,
    T: Into<Card>,
{
    fn from(iterable: I) -> Self {
        Self {
            src: iterable
                .into_iter()
                .fold(BTreeMap::new(), |mut acc, (t, mn)| {
                    let key = t.into();
                    let val = mn.into();
                    acc.entry(key)
                        .and_modify(|usage| {
                            *usage += val;
                        })
                        .or_insert(val);
                    acc
                }),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Usage {
    pub consumed: StockInt,
    pub retained: StockInt,
}

impl Usage {
    pub const fn needed(self) -> StockInt {
        self.consumed + self.retained
    }
}

impl From<(StockInt, StockInt)> for Usage {
    fn from((consumed, retained): (StockInt, StockInt)) -> Self {
        Self { consumed, retained }
    }
}

impl Add for Usage {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            consumed: self.consumed + rhs.consumed,
            retained: self.retained + rhs.retained,
        }
    }
}
impl AddAssign for Usage {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
