pub mod lane;
pub mod player;

use crate::{
    action::produce_or_barter::StockInt,
    card::{
        Card, Value,
        building::{
            Building, basic::BasicBuilding, normal::NormalBuilding, special::SpecialBuilding,
        },
        product1::Product1,
        product2::Product2,
        resource::Resource,
    },
};
use lane::Lane;
use player::Name;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use strum::IntoEnumIterator;

#[derive(Debug)]
pub struct State {
    /// `players.front()` represents the current player's turn.
    names: VecDeque<Name>,
    pub resource_lane: Lane<Resource>,
    pub product1_lane: Lane<Product1>,
    pub product2_lane: Lane<Product2>,
    pub building_lane: Lane<Building>,
}

impl State {
    pub fn all_cards(&self) -> BTreeMap<Card, StockInt> {
        let resources = Resource::iter();
        let product1 = Product1::iter();
        let product2 = Product2::iter();
        let basic_buildings = BasicBuilding::iter();
        let normal_buildings = NormalBuilding::iter();
        let special_buildings = SpecialBuilding::iter();

        resources
            .map(Card::from)
            .chain(product1.map(Into::<_>::into))
            .chain(product2.map(Into::<_>::into))
            .chain(basic_buildings.map(Into::<_>::into))
            .chain(normal_buildings.map(Into::<_>::into))
            .chain(special_buildings.map(Into::<_>::into))
            .map(|card| (card, card.total_n(self.names.len())))
            .collect()
    }

    pub fn curr_turn(&self) -> Option<Name> {
        self.names.front().copied()
    }

    pub fn any_deck_empty(&self) -> bool {
        self.resource_lane.is_deck_empty()
            || self.product1_lane.is_deck_empty()
            || self.product2_lane.is_deck_empty()
            || self.building_lane.is_deck_empty()
    }

    pub fn graveyard(&self, all_cards: &BTreeMap<Card, StockInt>) -> BTreeMap<Card, StockInt> {
        let resource_lane = self
            .resource_lane
            .slots_deck_union()
            .into_iter()
            .map(|(resource, n)| (resource.into(), n))
            .collect();
        let product1_lane = self
            .product1_lane
            .slots_deck_union()
            .into_iter()
            .map(|(product1, n)| (product1.into(), n))
            .collect();
        let product2_lane = self
            .product2_lane
            .slots_deck_union()
            .into_iter()
            .map(|(product2, n)| (product2.into(), n))
            .collect();
        let building_lane = self
            .building_lane
            .slots_deck_union()
            .into_iter()
            .map(|(building, n)| (building.into(), n))
            .collect();
        btree_map_sub_diff(
            &btree_map_sub_diff(
                &btree_map_sub_diff(
                    &btree_map_sub_diff(all_cards, &resource_lane),
                    &product1_lane,
                ),
                &product2_lane,
            ),
            &building_lane,
        )
    }

    pub fn fill_decks(&mut self) {
        Self::all_cards(&self)
    }
}

fn btree_map_add_union<T: Clone + Ord>(
    lhs: &BTreeMap<T, StockInt>,
    rhs: &BTreeMap<T, StockInt>,
) -> BTreeMap<T, StockInt> {
    lhs.keys()
        .chain(rhs.keys())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .filter_map(|key| {
            let lhs_val = lhs.get(key).copied().unwrap_or(0);
            let rhs_val = rhs.get(key).copied().unwrap_or(0);
            let sum = lhs_val.saturating_add(rhs_val);
            (0 < sum).then_some((key.clone(), sum))
        })
        .collect()
}

fn btree_map_sub_diff<T: Clone + Ord>(
    lhs: &BTreeMap<T, StockInt>,
    rhs: &BTreeMap<T, StockInt>,
) -> BTreeMap<T, StockInt> {
    lhs.iter()
        .filter_map(|(key, &vall)| {
            let diff = rhs.get(key).map_or(vall, |&valr| vall.saturating_sub(valr));
            (0 < diff).then_some((key.clone(), diff))
        })
        .collect()
}
