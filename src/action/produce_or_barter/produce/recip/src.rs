use super::super::super::RecipInt;
use crate::card::Card;
use std::{
    collections::BTreeMap,
    ops::{Add, AddAssign},
};

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Src {
    pub src: BTreeMap<Card, Usage>,
}

impl<T, I> From<I> for Src
where
    I: IntoIterator<Item = (T, (RecipInt, RecipInt))>,
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Usage {
    pub consumed: RecipInt,
    pub retained: RecipInt,
}

impl Usage {
    pub const fn needed(self) -> RecipInt {
        self.consumed + self.retained
    }
}

impl From<(RecipInt, RecipInt)> for Usage {
    fn from((consumed, retained): (RecipInt, RecipInt)) -> Self {
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
