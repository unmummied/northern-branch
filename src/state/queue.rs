pub mod player;

use crate::{
    action::produce_or_barter::StockInt,
    card::{
        Card, CardInfo,
        building::{basic::BasicBuilding, normal::NormalBuilding, special::SpecialBuilding},
        product1::Product1,
        product2::Product2,
        resource::Resource,
    },
};
use player::Name;
use std::collections::{BTreeMap, VecDeque};
use strum::IntoEnumIterator;

type QueueInt = u8;
#[derive(Debug)]
pub struct Queue {
    queue: VecDeque<Name>,
}

impl Queue {
    pub fn all_using_cards(&self) -> BTreeMap<Card, StockInt> {
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
            .map(|card| (card, card.total_n(self.queue.len())))
            .collect()
    }

    pub fn curr_turn(&self) -> Option<Name> {
        self.queue.front().copied()
    }

    pub fn rotate_turn(&mut self) {
        if let Some(curr) = self.queue.pop_front() {
            self.queue.push_back(curr);
        }
    }
}

impl<T, I> From<I> for Queue
where
    I: IntoIterator<Item = T>,
    T: Into<Name>,
{
    fn from(queue: I) -> Self {
        Self {
            queue: queue.into_iter().map(Into::into).collect(),
        }
    }
}
