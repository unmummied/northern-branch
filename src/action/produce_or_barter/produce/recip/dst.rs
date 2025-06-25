use super::super::super::StockInt;
use crate::card::Card;
use std::collections::BTreeMap;

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Dst {
    dst: BTreeMap<Card, StockInt>,
}

impl<T, I> From<I> for Dst
where
    I: IntoIterator<Item = (T, StockInt)>,
    T: Into<Card>,
{
    fn from(iterable: I) -> Self {
        Self {
            dst: iterable
                .into_iter()
                .fold(BTreeMap::new(), |mut acc, (t, n)| {
                    let key = t.into();
                    acc.entry(key)
                        .and_modify(|lhs| {
                            *lhs += n;
                        })
                        .or_insert(n);
                    acc
                }),
        }
    }
}
