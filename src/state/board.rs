pub mod lane;

use crate::card::{
    Card, building::Building, product1::Product1, product2::Product2, resource::Resource,
};
use lane::Lane;
use rand::Rng;

#[derive(Debug, Default)]
#[allow(clippy::struct_field_names)]
pub struct BoardState {
    resource_lane: Lane<Resource>,
    product1_lane: Lane<Product1>,
    product2_lane: Lane<Product2>,
    building_lane: Lane<Building>,
}

impl BoardState {
    // Getters
    pub const fn resource_lane(&self) -> &Lane<Resource> {
        &self.resource_lane
    }
    pub const fn product1_lane(&self) -> &Lane<Product1> {
        &self.product1_lane
    }
    pub const fn product2_lane(&self) -> &Lane<Product2> {
        &self.product2_lane
    }
    pub const fn building_lane(&self) -> &Lane<Building> {
        &self.building_lane
    }

    pub fn discard(&mut self, card: Card) {
        match card {
            Card::Resource(resource) => self.resource_lane.discard(resource),
            Card::Product1(product1) => self.product1_lane.discard(product1),
            Card::Product2(product2) => self.product2_lane.discard(product2),
            Card::Building(building) => self.building_lane.discard(building),
            Card::VictoryPoint => unreachable!(), // victory points card is never discard.
        }
    }

    pub fn fill_slots<R: Rng>(&mut self, rng: &mut R) {
        self.resource_lane.fill_slots(rng);
        self.product1_lane.fill_slots(rng);
        self.product2_lane.fill_slots(rng);
        self.building_lane.fill_slots(rng);
    }
}
