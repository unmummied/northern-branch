use super::CARD_WIDTH;
use crate::{action::produce_or_barter::StockInt, card::building::Building};
use anyhow::{Context, anyhow};
use fancy_regex::Regex;
use rand::{
    Rng,
    distr::{Distribution, weighted::WeightedIndex},
};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::{self, Display, Formatter},
};

const TOO_FEW_CARDS_ERR: &str = "too few cards...";
const TOO_MUCH_CARDS_ERR: &str = "too much cards...";
const CARD_NOT_IN_SLOT: &str = "the card is not in the slot...";

#[derive(Debug, Default, Clone)]
pub struct Lane<T> {
    pub slots: [(T, StockInt); 5],
    deck: Option<BTreeMap<T, StockInt>>,
    discard_pile: Option<BTreeMap<T, StockInt>>,
}

impl<T: Default + Clone + Ord> Lane<T> {
    pub fn new() -> Self {
        Self {
            slots: Self::default().slots,
            deck: Some(BTreeMap::default()),
            discard_pile: Some(BTreeMap::default()),
        }
    }
    pub fn new_discard_pile_unuse() -> Self {
        Self {
            slots: Self::default().slots,
            deck: Some(BTreeMap::default()),
            discard_pile: None,
        }
    }
    pub fn from_slots_only<I: IntoIterator<Item = T>>(iterable: I) -> anyhow::Result<Self> {
        let mut iter = iterable.into_iter();
        Ok(Self {
            slots: [
                (iter.next().context(TOO_FEW_CARDS_ERR)?, 0),
                (iter.next().context(TOO_FEW_CARDS_ERR)?, 0),
                (iter.next().context(TOO_FEW_CARDS_ERR)?, 0),
                (iter.next().context(TOO_FEW_CARDS_ERR)?, 0),
                (iter.next().context(TOO_FEW_CARDS_ERR)?, 0),
            ],
            deck: None,
            discard_pile: None,
        })
    }

    // Getters
    pub const fn deck(&self) -> Option<&BTreeMap<T, StockInt>> {
        self.deck.as_ref()
    }
    pub const fn discard_pile(&self) -> Option<&BTreeMap<T, StockInt>> {
        self.discard_pile.as_ref()
    }

    // Length
    fn len_deck(&self) -> Option<StockInt> {
        self.deck().map(|map| map.values().sum())
    }
    fn len_discard_pile(&self) -> Option<StockInt> {
        self.discard_pile().map(|map| map.values().sum())
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
    pub fn is_deck_empty(&self) -> Option<bool> {
        self.deck().map(BTreeMap::is_empty)
    }
    /// Returns `true` if the discard pile is empty.
    pub fn is_discard_pile_empty(&self) -> Option<bool> {
        self.discard_pile().map(BTreeMap::is_empty)
    }

    /// If the `slot` contains the `card` and one or more stocks exist,
    /// the stock is returned.
    ///
    /// This method never returns `Some(0)`.
    pub fn stock_in_slot(&self, card: &T) -> Option<StockInt> {
        if let Some(idx) = self.slot_idx(card) {
            if let Some((_, stock)) = self.slots.get(idx) {
                // If `slot_idx` returns `Some(idx)`, then `idx` is never out of bounds.
                // Thus, this `if let` statement is redundant.
                // This code is equivalent to the following code:
                //
                // let (_, stock) = self.slots[idx];
                // return Some(stock);
                return Some(*stock);
            }
        }
        None
    }

    pub fn is_slot_in_n(&self, card: &T, n: StockInt) -> bool {
        if let Some(stock) = self.stock_in_slot(card) {
            return stock <= n;
        }
        false
    }

    pub fn slot_out_clone(&self, card: &T, n: StockInt) -> Result<Self, &'static str> {
        let slots = self.slots.clone();
        let mut res = self.clone();
        if let Some(idx) = self.slot_idx(card) {
            let (already_in, stock) = &slots[idx];
            res.slots[idx] = (already_in.clone(), stock - n);
            return Ok(res);
        }
        Err(CARD_NOT_IN_SLOT)
    }

    /// Discards the given card by adding it to the `discarded pile`.
    pub fn discard_n(&mut self, card: T, n: StockInt) {
        if let Some(map) = self.discard_pile.as_mut() {
            map.entry(card).and_modify(|m| *m += n).or_insert(n);
            return;
        }
        if let Some(map) = self.deck.as_mut() {
            map.entry(card).and_modify(|m| *m += n).or_insert(n);
            return;
        }
        if let Some(idx) = self.slot_idx(&card) {
            self.slots[idx].1 += n;
        }
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
        let deck = self.deck.as_mut()?;

        let weights = deck.values();
        let dist = WeightedIndex::new(weights).ok()?;

        let chosen_idx = dist.sample(rng);

        // Note: .nth(n) on BTreeMap::keys() is O(n), but acceptable here.
        // because the deck size is expected to remain small.
        // This trades a bit of performance for simplicity and low memory overhead.
        let chosen = deck.keys().nth(chosen_idx).cloned();

        if let Some(ref card) = chosen {
            if let Some(cnt) = deck.get_mut(card) {
                *cnt = cnt.saturating_sub(1);
                if *cnt == 0 {
                    deck.remove(card);
                }
            }
        }

        chosen
    }

    /// Fill deck by discard pile.
    fn fill_deck(&mut self) {
        if !(self.is_deck_empty().unwrap_or(true)) {
            return;
        }
        self.deck = self.discard_pile.clone();
        self.discard_pile.as_mut().map(BTreeMap::clear);
    }

    /// Fill slots from deck.
    ///
    /// Returns `true` if the slot is completely filled,
    /// `false` if there is still a vacant slot but the deck is empty and cannot be filled.
    fn fill_slots_from_deck<R: Rng>(&mut self, rng: &mut R) -> bool {
        while let Some(vacant) = self.vacant_slot() {
            if let Some(ref chosen) = self.draw(rng) {
                if let Some(idx) = self.slot_idx(chosen) {
                    self.slots[idx].1 += 1;
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

impl Lane<Building> {
    /// A constractor for building lane.
    pub fn from_discard_pile_unuse_with_init_subslots_and_deck<I, J>(
        subslots: I,
        buildings_deck: J,
    ) -> anyhow::Result<Self>
    where
        I: IntoIterator<Item = Building>,
        J: IntoIterator<Item = (Building, StockInt)>,
    {
        let subslots = subslots.into_iter();
        let mut slots = Self::default().slots;
        let mut chosen = BTreeSet::new();
        for (idx, basic) in subslots.enumerate() {
            if slots.len() <= idx {
                return Err(anyhow!(TOO_MUCH_CARDS_ERR));
            }
            slots[idx] = (basic, 1);
            chosen.insert(basic);
        }
        let complement = buildings_deck
            .into_iter()
            .filter(|(building, _)| !chosen.contains(building))
            .collect();
        Ok(Self {
            slots,
            deck: Some(complement),
            discard_pile: None,
        })
    }
}

impl<T: Default + Clone + Ord + Display> Display for Lane<T> {
    #[allow(clippy::cast_possible_truncation)]
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

        cnts.push(self.len_deck().unwrap_or_default() as _);
        row1s.push(String::new());
        row2s.push("???".into());
        row3s.push(String::new());
        cnts.push(self.len_discard_pile().unwrap_or_default() as _);
        row1s.push(String::new());
        row2s.push("xxx".into());
        row3s.push(String::new());

        let is_deck_exist = self.deck().is_some();
        let is_discard_pile_exist = self.discard_pile().is_some();

        write!(
            f,
            "{upper} {} {upper} {} {upper} {} {upper} {} {upper} {}",
            cnts[0], cnts[1], cnts[2], cnts[3], cnts[4]
        )?;
        if is_deck_exist {
            write!(f, " | {upper}{:>2}", cnts[5])?;
            if is_discard_pile_exist {
                write!(f, " {upper}{:>2}", cnts[6])?;
            }
        }
        writeln!(f)?;
        write!(
            f,
            "|{:^width$}| |{:^width$}| |{:^width$}| |{:^width$}| |{:^width$}|",
            row1s[0], row1s[1], row1s[2], row1s[3], row1s[4]
        )?;
        if is_deck_exist {
            write!(f, " | |{:^width$}|", row1s[5])?;
            if is_discard_pile_exist {
                write!(f, " |{:^width$}|", row1s[6])?;
            }
        }
        writeln!(f)?;
        write!(
            f,
            "|{:^width$}| |{:^width$}| |{:^width$}| |{:^width$}| |{:^width$}|",
            row2s[0], row2s[1], row2s[2], row2s[3], row2s[4]
        )?;
        if is_deck_exist {
            write!(f, " | |{:^width$}|", row2s[5])?;
            if is_discard_pile_exist {
                write!(f, " |{:^width$}|", row2s[6])?;
            }
        }
        writeln!(f)?;
        write!(
            f,
            "|{:^width$}| |{:^width$}| |{:^width$}| |{:^width$}| |{:^width$}|",
            row3s[0], row3s[1], row3s[2], row3s[3], row3s[4]
        )?;
        if is_deck_exist {
            write!(f, " | |{:^width$}|", row3s[5])?;
            if is_discard_pile_exist {
                write!(f, " |{:^width$}|", row3s[6])?;
            }
        }
        writeln!(f)?;
        write!(f, "{lower} {lower} {lower} {lower} {lower}")?;
        if is_deck_exist {
            write!(f, " | {lower}")?;
            if is_discard_pile_exist {
                write!(f, " {lower}")?;
            }
        }
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
