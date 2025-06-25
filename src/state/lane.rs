use super::btree_map_add_union;
use crate::action::produce_or_barter::StockInt;
use rand::{
    distr::{Distribution, weighted::WeightedIndex},
    rng,
};
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Lane<T> {
    slots: [(T, StockInt); 5],
    deck: BTreeMap<T, StockInt>,
    graveyard: BTreeMap<T, StockInt>,
}

impl<T: Clone + Ord> Lane<T> {
    // Getters
    pub fn slots(&self) -> &[(T, StockInt); 5] {
        &self.slots
    }
    pub fn deck(&self) -> &BTreeMap<T, StockInt> {
        &self.deck
    }
    pub fn graveyard(&self) -> &BTreeMap<T, StockInt> {
        &self.graveyard
    }

    /// Returns the index of a single empty slot, even if multiple exist.
    fn vacant_slot(&self) -> Option<usize> {
        self.slots
            .iter()
            .enumerate()
            .find(|(_, (_, n))| *n == 0)
            .map(|(i, _)| i)
    }
    /// Returns `true` if the deck is empty.
    pub fn is_deck_empty(&self) -> bool {
        self.deck.is_empty()
    }
    pub fn is_graveyard_empty(&self) -> bool {
        self.graveyard().is_empty()
    }

    /// Returns the union of cards in the slots and in the deck.
    pub fn slots_deck_union(&self) -> BTreeMap<T, StockInt> {
        btree_map_add_union(&self.slots.clone().into(), &self.deck)
    }

    /// Returns the index of the slot if the card is already present.
    fn slot_idx(&self, card: &T) -> Option<usize> {
        self.slots
            .iter()
            .enumerate()
            .find(|(_, (already_in, _))| already_in == card)
            .map(|(i, _)| i)
    }

    /// Draw a random card from the deck (and removing it), if the deck is not empty.
    fn draw(&mut self) -> Option<T> {
        let weights = self.deck.values();

        let dist = WeightedIndex::new(weights).ok()?;
        let mut rng = rng();

        let chosen_idx = dist.sample(&mut rng);
        let chosen = self.deck.keys().nth(chosen_idx).cloned();
        if let Some(ref card) = chosen {
            if let Some(cnt) = self.deck.get_mut(card) {
                *cnt = cnt.saturating_sub(1);
                if *cnt == 0 {
                    self.deck.remove(card);
                }
            }
        }

        chosen
    }

    /// Fill slots as much as possible.
    ///
    /// Returns `true` if the slot is completely filled,
    /// `false` if there is still a vacant slot but the deck is empty and cannot be filled.
    pub fn fill_slots_from_deck(&mut self) -> bool {
        while let Some(vacant) = self.vacant_slot() {
            if let Some(ref chosen) = self.draw() {
                if let Some(already_in) = self.slot_idx(chosen) {
                    self.slots[already_in].1 += 1;
                } else {
                    self.slots[vacant] = (chosen.clone(), 1);
                }
            } else {
                return false;
            }
        }
        true
    }
}
