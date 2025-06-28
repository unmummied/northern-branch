pub mod lane;

use super::PopulationInt;
use crate::card::{
    Card, building::Building, product1::Product1, product2::Product2, resource::Resource,
};
use lane::Lane;
use rand::Rng;
use std::{
    fmt::{self, Display, Formatter},
    iter,
};

const CARD_WIDTH: usize = 11;

#[derive(Debug, Default)]
#[allow(clippy::struct_field_names)]
pub struct BoardState {
    resource_lane: Lane<Resource>,
    product1_lane: Lane<Product1>,
    product2_lane: Lane<Product2>,
    building_lane: Lane<Building>,
}

impl BoardState {
    pub fn new_n(population: PopulationInt) -> Result<Self, &'static str> {
        let mut res = Self::default();
        Card::deck(population)?
            .into_iter()
            .flat_map(|(card, n)| iter::repeat_n(card, n as _))
            .for_each(|card| res.discard(card));
        Ok(res)
    }

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

impl Display for BoardState {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let width = CARD_WIDTH + 2;
        writeln!(
            f,
            "    {:^width$} {:^width$} {:^width$} {:^width$} {:^width$} | {:^width$} {:^width$}",
            1, 2, 3, 4, 5, "Deck", "Discard"
        )?;
        writeln!(
            f,
            "{}",
            prefix_each_line(
                &self.building_lane.to_string(),
                &["    ", "    ", "B:  ", "    ", "    "]
            )
        )?;
        writeln!(
            f,
            "{}",
            prefix_each_line(
                &self.product2_lane.to_string(),
                &["    ", "    ", "P2: ", "    ", "    "]
            )
        )?;
        writeln!(
            f,
            "{}",
            prefix_each_line(
                &self.product1_lane.to_string(),
                &["    ", "    ", "P1: ", "    ", "    "]
            )
        )?;
        write!(
            f,
            "{}",
            prefix_each_line(
                &self.resource_lane.to_string(),
                &["    ", "    ", "R:  ", "    ", "    "]
            )
        )?;
        Ok(())
    }
}

#[must_use]
fn prefix_each_line(text: &str, prefixes: &[&str]) -> String {
    text.lines()
        .enumerate()
        .map(|(i, line)| {
            let prefix = prefixes.get(i).copied().unwrap_or("");
            format!("{prefix}{line}")
        })
        .collect::<Vec<_>>()
        .join("\n")
}
