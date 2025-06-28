use super::CARD_WIDTH;
use crate::action::produce_or_barter::StockInt;
use fancy_regex::Regex;
use rand::{
    Rng,
    distr::{Distribution, weighted::WeightedIndex},
};
use std::{
    collections::BTreeMap,
    fmt::{self, Display, Formatter},
};

#[derive(Debug, Default)]
pub struct Lane<T> {
    slots: [(T, StockInt); 5],
    deck: BTreeMap<T, StockInt>,
    discard_pile: BTreeMap<T, StockInt>,
}

impl<T: Clone + Ord> Lane<T> {
    // Getters
    pub const fn slots(&self) -> &[(T, StockInt); 5] {
        &self.slots
    }
    pub const fn deck(&self) -> &BTreeMap<T, StockInt> {
        &self.deck
    }
    pub const fn discard_pile(&self) -> &BTreeMap<T, StockInt> {
        &self.discard_pile
    }

    /// Discards the given card by adding it to the `discarded pile`.
    ///
    /// If the card is already present, increments its count by 1.
    /// Otherwise, inserts it with a count of 1.
    pub fn discard(&mut self, card: T) {
        self.discard_pile
            .entry(card)
            .and_modify(|n| *n += 1)
            .or_insert(1);
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
    /// Returns `true` if the discard pile is empty.
    pub fn is_discard_pile_empty(&self) -> bool {
        self.discard_pile().is_empty()
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
    fn draw<R: Rng>(&mut self, rng: &mut R) -> Option<T> {
        let weights = self.deck.values();

        let dist = WeightedIndex::new(weights).ok()?;

        let chosen_idx = dist.sample(rng);

        // Note: .nth(n) on BTreeMap::keys() is O(n), but acceptable here.
        // because the deck size is expected to remain small.
        // This trades a bit of performance for simplicity and low memory overhead.
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

    /// Fill deck by discard pile.
    fn fill_deck(&mut self) {
        if !self.is_deck_empty() {
            return;
        }
        self.deck = self.discard_pile.clone();
        self.discard_pile.clear();
    }

    /// Fill slots from deck.
    ///
    /// Returns `true` if the slot is completely filled,
    /// `false` if there is still a vacant slot but the deck is empty and cannot be filled.
    fn fill_slots_from_deck<R: Rng>(&mut self, rng: &mut R) -> bool {
        while let Some(vacant) = self.vacant_slot() {
            if let Some(ref chosen) = self.draw(rng) {
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

    /// Fill slots as mush as possible.
    pub fn fill_slots<R: Rng>(&mut self, rng: &mut R) {
        if self.fill_slots_from_deck(rng) {
            return;
        }
        self.fill_deck();
        self.fill_slots_from_deck(rng);
    }
}

impl<T: Clone + Display> Display for Lane<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let width = CARD_WIDTH;
        let upper = format!("+{}-", "-".repeat(width - 2));
        let lower = format!("+{}+", "-".repeat(width));

        let mut cnts = Vec::new();
        let mut row1s = Vec::new();
        let mut row2s = Vec::new();
        let mut row3s = Vec::new();
        for (card, n) in self.slots.clone() {
            let (row1, row2, row3) =
                into_three_rows_style(&card.to_string()).map_err(|_| fmt::Error)?;
            cnts.push(n);
            row1s.push(row1);
            row2s.push(row2);
            row3s.push(row3);
        }

        writeln!(
            f,
            "{upper} {} {upper} {} {upper} {} {upper} {} {upper} {} |",
            cnts[0], cnts[1], cnts[2], cnts[3], cnts[4]
        )?;
        writeln!(
            f,
            "|{:<width$}| |{:<width$}| |{:<width$}| |{:<width$}| |{:<width$}| |",
            row1s[0], row1s[1], row1s[2], row1s[3], row1s[4]
        )?;
        writeln!(
            f,
            "|{:<width$}| |{:<width$}| |{:<width$}| |{:<width$}| |{:<width$}| |",
            row2s[0], row2s[1], row2s[2], row2s[3], row2s[4]
        )?;
        writeln!(
            f,
            "|{:<width$}| |{:<width$}| |{:<width$}| |{:<width$}| |{:<width$}| |",
            row3s[0], row3s[1], row3s[2], row3s[3], row3s[4]
        )?;
        write!(f, "{lower} {lower} {lower} {lower} {lower} |")?;
        Ok(())
    }
}

/// Splits a CamelCase string into up to three rows, respecting word boundaries and width.
pub fn into_three_rows_style(upper_camel: &str) -> anyhow::Result<(String, String, String)> {
    let mut words = split_upper_camel(upper_camel)?;

    let r1 = words.next();
    let r2 = words.next();
    let r3 = words.next();
    let (r1, r2, r3) = match (r1, r2, r3) {
        (Some(w1), None, _) => (String::default(), w1, String::default()),
        (w1, w2, w3) => (
            w1.unwrap_or_default(),
            w2.unwrap_or_default(),
            w3.unwrap_or_default(),
        ),
    };

    Ok((r1, r2, r3))
}

pub fn split_upper_camel(upper_camel: &str) -> anyhow::Result<impl Iterator<Item = String>> {
    let re = Regex::new(r"[A-Z]{2,}(?=[A-Z][a-z])|[A-Z][a-z0-9]+|[A-Z]+")?;
    let matches = re.find_iter(upper_camel).collect::<Result<Vec<_>, _>>()?;
    Ok(matches.into_iter().map(|mat| mat.as_str().to_string()))
}
