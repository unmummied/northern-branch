use crate::{
    action::produce_or_barter::StockInt,
    card::{Card, VictInt, building::Building},
};
use std::collections::BTreeMap;

#[derive(Debug, Default)]
pub struct Inventory {
    cards: BTreeMap<Card, StockInt>,
    buildings: BTreeMap<Building, StockInt>, // if building is unique, this is redundant.
    victory_points: VictInt,
}
