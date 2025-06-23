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
}

impl<T: Clone + Ord> Lane<T> {
    pub fn is_deck_empty(&self) -> bool {
        self.deck.is_empty()
    }

    pub fn slots_and_deck(&self) -> BTreeMap<T, StockInt> {
        todo!()
    }

    fn vacant_slot(&self) -> Option<usize> {
        self.slots
            .iter()
            .enumerate()
            .find(|(_, (_, n))| *n == 0)
            .map(|(i, _)| i)
    }

    fn already_in_slot(&self, card: &T) -> Option<usize> {
        self.slots
            .iter()
            .enumerate()
            .find(|(_, (already_in, _))| already_in == card)
            .map(|(i, _)| i)
    }

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
    /// Returns true if the slot is completely filled,
    /// false if there is still a vacant slot but the deck is empty and cannot be filled.
    pub fn fill_slots(&mut self) -> bool {
        while let Some(vacant) = self.vacant_slot() {
            if let Some(ref chosen) = self.draw() {
                if let Some(already_in) = self.already_in_slot(chosen) {
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

fn f() {
    todo!()
}
