use crate::{
    action::produce_or_barter::{
        StockInt,
        produce::{
            Recip,
            recip::{
                RecipBy,
                dst::Dst,
                src::{Src, Usage},
            },
        },
    },
    card::{Card, VictInt, building::Building},
};
use std::collections::BTreeMap;

const UNKNOWN_RECIP: &str = "unknown recip..";
const INSUFFICIENT_SRC: &str = "src is insufficient...";

#[derive(Debug, Default, Clone)]
pub struct Inventory {
    pub cards: BTreeMap<Card, StockInt>,
    pub buildings: BTreeMap<Building, StockInt>, // if building is unique, this is redundant.
    pub victory_points: VictInt,
}

impl Inventory {
    fn is_subset(&self, superset: &Self) -> bool {
        is_subset(&self.cards, &superset.cards)
            && is_subset(&self.buildings, &superset.buildings)
            && self.victory_points <= superset.victory_points
    }

    fn union(&self, other: &Self) -> Self {
        Self {
            cards: union(&self.cards, &other.cards),
            buildings: union(&self.buildings, &other.buildings),
            victory_points: self.victory_points + other.victory_points,
        }
    }

    fn difference(&self, other: &Self) -> Self {
        Self {
            cards: difference(&self.cards, &other.cards),
            buildings: difference(&self.buildings, &other.buildings),
            victory_points: self.victory_points.saturating_sub(other.victory_points),
        }
    }

    pub fn try_produce_clone(
        &self,
        recip: &Recip,
        book: &RecipBy<Src, Dst>,
    ) -> Result<Self, &'static str> {
        if !recip.is_in(book) {
            return Err(UNKNOWN_RECIP);
        }
        let (consumed, retained) = recip.src.clone().into();
        let src = consumed.union(&retained);
        if !src.is_subset(self) {
            return Err(INSUFFICIENT_SRC);
        }
        let dst = recip.dst.clone().into();
        Ok(self.difference(&consumed).union(&dst))
    }
}

impl From<Src> for (Inventory, Inventory) {
    fn from(src: Src) -> Self {
        let mut consumed_others = BTreeMap::new();
        let mut consumed_buildings = BTreeMap::new();
        let mut consumed_victory_points = 0;
        let mut retained_others = BTreeMap::new();
        let mut retained_buildings = BTreeMap::new();
        let mut retained_victory_points = 0;
        src.src
            .into_iter()
            .for_each(|(card, Usage { consumed, retained })| match card {
                Card::Building(building) => {
                    consumed_buildings.insert(building, consumed);
                    retained_buildings.insert(building, retained);
                }
                Card::OneVictoryPoint => {
                    consumed_victory_points += consumed;
                    retained_victory_points += retained;
                }
                _ => {
                    consumed_others.insert(card, consumed);
                    retained_others.insert(card, retained);
                }
            });
        (
            Inventory {
                cards: consumed_others,
                buildings: consumed_buildings,
                victory_points: consumed_victory_points,
            },
            Inventory {
                cards: retained_others,
                buildings: retained_buildings,
                victory_points: retained_victory_points,
            },
        )
    }
}

impl From<Dst> for Inventory {
    fn from(dst: Dst) -> Self {
        let mut others = BTreeMap::new();
        let mut buildings = BTreeMap::new();
        let mut victory_points = 0;
        dst.dst.into_iter().for_each(|(card, n)| match card {
            Card::Building(building) => {
                buildings.insert(building, n);
            }
            Card::OneVictoryPoint => {
                victory_points += n;
            }
            _ => {
                others.insert(card, n);
            }
        });
        Self {
            cards: others,
            buildings,
            victory_points,
        }
    }
}

fn is_subset<K: Ord>(subset: &BTreeMap<K, StockInt>, superset: &BTreeMap<K, StockInt>) -> bool {
    subset
        .iter()
        .all(|(key, value)| *value <= superset.get(key).copied().unwrap_or_default())
}

fn union<K: Clone + Ord>(
    lhs: &BTreeMap<K, StockInt>,
    rhs: &BTreeMap<K, StockInt>,
) -> BTreeMap<K, StockInt> {
    lhs.keys()
        .chain(rhs.keys())
        .map(|key| {
            (
                key.clone(),
                lhs.get(key).copied().unwrap_or_default()
                    + rhs.get(key).copied().unwrap_or_default(),
            )
        })
        .collect()
}

fn difference<K: Clone + Ord>(
    lhs: &BTreeMap<K, StockInt>,
    rhs: &BTreeMap<K, StockInt>,
) -> BTreeMap<K, StockInt> {
    lhs.iter()
        .filter_map(|(key, value)| {
            let diff = value.saturating_sub(rhs.get(key).copied().unwrap_or_default());
            (0 < diff).then(|| (key.clone(), diff))
        })
        .collect()
}
