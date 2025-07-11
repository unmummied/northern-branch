pub mod lane;

use super::PopulationInt;
use crate::{
    action::produce_or_barter::{
        StockInt,
        produce::recip::{dst::Dst, src::Src},
    },
    card::{
        Card, Quantity,
        building::{Building, basic::BasicBuilding},
        product1::Product1,
        product2::Product2,
        resource::Resource,
    },
};
use anyhow::anyhow;
use lane::Lane;
use rand::Rng;
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::{self, Display, Formatter},
};
use strum::IntoEnumIterator;

const CARD_WIDTH: usize = 11;
const ERR_INVALID_DST: &str = "invalid dst...";

#[derive(Debug, Default, Clone)]
#[allow(clippy::struct_field_names)]
pub struct BoardState {
    resource_lane: Lane<Resource>,
    product1_lane: Lane<Product1>,
    product2_lane: Lane<Product2>,
    building_lane: Lane<Building>,
}

impl BoardState {
    pub fn with_deal<R: Rng>(rng: &mut R, population: PopulationInt) -> anyhow::Result<Self> {
        let chosen_basics = BasicBuilding::chosen_basics(rng, population);
        let mut res = Self {
            resource_lane: Lane::from_slots_only(Resource::iter())?,
            product1_lane: Lane::new(),
            product2_lane: Lane::new(),
            building_lane: Lane::from_discard_pile_unuse_with_init_subslots_and_deck(
                chosen_basics.map(Into::into),
                Building::all_iter()
                    .map(|building| building.quantity(population).map(|n| (building, n)))
                    .collect::<Result<BTreeSet<_>, _>>()
                    .map_err(|e| anyhow!(e))?,
            )?,
        };
        Card::deck(population)
            .map_err(|e| anyhow!(e))?
            .into_iter()
            .filter(|(card, _)| !card.is_building())
            .for_each(|(card, n)| res.discard_n(card, n));
        res.fill_slots(rng);
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

    fn is_slot_in_n(&self, card: Card, n: StockInt) -> bool {
        match card {
            Card::Resource(resource) => self.resource_lane.is_slot_in_n(&resource, n),
            Card::Product1(product1) => self.product1_lane.is_slot_in_n(&product1, n),
            Card::Product2(product2) => self.product2_lane.is_slot_in_n(&product2, n),
            Card::Building(building) => self.building_lane.is_slot_in_n(&building, n),
            Card::OneVictoryPoint => true,
        }
    }

    pub fn contains(&self, dst: &Dst) -> bool {
        dst.dst.iter().all(|(&card, &n)| self.is_slot_in_n(card, n))
    }

    pub fn try_produce_clone<R: Rng>(&self, rng: &mut R, dst: &Dst) -> Result<Self, &'static str> {
        let mut res = self.clone();
        for (card, n) in &dst.dst {
            match card {
                Card::Resource(resource) => {
                    res.resource_lane = res.resource_lane.slot_out_clone(resource, *n)?;
                }
                Card::Product1(product1) => {
                    res.product1_lane = res.product1_lane.slot_out_clone(product1, *n)?;
                }
                Card::Product2(product2) => {
                    res.product2_lane = res.product2_lane.slot_out_clone(product2, *n)?;
                }
                Card::Building(building) => {
                    res.building_lane = res.building_lane.slot_out_clone(building, *n)?;
                }
                Card::OneVictoryPoint => {
                    return Err(ERR_INVALID_DST);
                }
            }
        }
        res.fill_slots(rng);
        Ok(res)
    }
    pub fn try_barter_clone<R: Rng>(
        &self,
        rng: &mut R,
        taken: &BTreeMap<Card, StockInt>,
    ) -> Result<Self, &'static str> {
        let mut res = self.clone();
        for (card, n) in taken {
            match card {
                Card::Resource(resource) => {
                    res.resource_lane = res.resource_lane.slot_out_clone(resource, *n)?;
                }
                Card::Product1(product1) => {
                    res.product1_lane = res.product1_lane.slot_out_clone(product1, *n)?;
                }
                Card::Product2(product2) => {
                    res.product2_lane = res.product2_lane.slot_out_clone(product2, *n)?;
                }
                Card::Building(building) => {
                    res.building_lane = res.building_lane.slot_out_clone(building, *n)?;
                }
                Card::OneVictoryPoint => {
                    return Err(ERR_INVALID_DST);
                }
            }
        }
        res.fill_slots(rng);
        Ok(res)
    }

    pub fn discard_n(&mut self, card: Card, n: StockInt) {
        match card {
            Card::Resource(resource) => self.resource_lane.discard_n(resource, n),
            Card::Product1(product1) => self.product1_lane.discard_n(product1, n),
            Card::Product2(product2) => self.product2_lane.discard_n(product2, n),
            Card::Building(building) => self.building_lane.discard_n(building, n),
            Card::OneVictoryPoint => unreachable!(), // victory points card is never discard.
        }
    }
    pub fn discard_src(&mut self, src: &Src) {
        src.src.iter().for_each(|(&card, &usage)| {
            self.discard_n(card, usage.consumed);
        });
    }
    pub fn discard_given(&mut self, given: &BTreeMap<Card, StockInt>) {
        for (&card, &n) in given {
            self.discard_n(card, n);
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
            1, 2, 3, 4, 5, "Deck", "Discarded"
        )?;
        writeln!(
            f,
            "{}",
            prefix_each_line(
                &self.building_lane.to_string(),
                &["    ", "    ", "B : ", "    ", "    "]
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
                &["    ", "    ", "R : ", "    ", "    "]
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
