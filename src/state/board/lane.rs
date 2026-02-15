use crate::{
    action::produce_or_barter::StockInt,
    card::{Quantity, VP_DISPLAY, Value, building::Building},
};
use anyhow::anyhow;
use rand::{
    Rng,
    distr::{Distribution, weighted::WeightedIndex},
};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::{self, Display, Formatter},
};

/// Number of slots. Must always match the number of variants in the `Resource` enum.
/// If you add or remove a variant from `Resource`, update this constant accordingly.
const SLOTS_COL: usize = 5;
const ERR_TOO_FEW_CARDS: &str = "too few cards...";
const ERR_TOO_MUCH_SUBSLOTS: &str = "too much subslots...";
const ERR_CARD_NOT_IN_SLOT: &str = "the card is not in the slot...";
const CARD_NAMES_MAX_LEN: usize = 25; // Building Material Factory

#[derive(Debug, Default, Clone)]
pub struct Lane<T> {
    slots: [(T, StockInt); SLOTS_COL],
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
        let slots = iterable
            .into_iter()
            .take(SLOTS_COL)
            .map(|card| (card, 0))
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_| anyhow!(ERR_TOO_FEW_CARDS))?;
        Ok(Self {
            slots,
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
        if let Some(idx) = self.slot_idx(card)
            && let Some((_, stock)) = self.slots.get(idx)
        {
            // If `slot_idx` returns `Some(idx)`, then `idx` is never out of bounds.
            // Thus, this `if let` statement is redundant.
            // This code is equivalent to the following code:
            //
            // let (_, stock) = self.slots[idx];
            // return Some(stock);
            return Some(*stock);
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
        if let Some(idx) = res.slot_idx(card) {
            let (already_in, stock) = &slots[idx];
            res.slots[idx] = (already_in.clone(), stock - n);
            return Ok(res);
        }
        Err(ERR_CARD_NOT_IN_SLOT)
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

        if let Some(ref card) = chosen
            && let Some(cnt) = deck.get_mut(card)
        {
            *cnt = cnt.saturating_sub(1);
            if *cnt == 0 {
                deck.remove(card);
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
            slots
                .get_mut(idx)
                .ok_or_else(|| anyhow!(ERR_TOO_MUCH_SUBSLOTS))
                .map(|slot| {
                    *slot = (basic, 1);
                })?;
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

impl<T: Default + Clone + Ord + Display + Value + Quantity> Display for Lane<T> {
    /// # Example
    ///
    /// ```
    /// +-----+-------------------------+----+-------+-------+
    /// | No. | Name                    | VP | Price | Stock |
    /// +-----+-------------------------+----+-------+-------+
    /// |   0 | Dung                    |  0 |    -1 |     6 |
    /// |   1 | Clay                    |  0 |     1 |     4 |
    /// |   2 | Barley                  |  0 |     1 |     3 |
    /// |   3 | Wood                    |  0 |     1 |     3 |
    /// |   4 | Ore                     |  0 |     1 |     3 |
    /// +-----+-------------------------+----+-------+-------+
    /// ```
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let (w0, w1, w2, w3, w4) = (3, CARD_NAMES_MAX_LEN, 2.max(VP_DISPLAY.len()), 5, 5);
        let bar = |f: &mut Formatter| {
            write!(
                f,
                "+-{empty:->w0$}-+-{empty:-<w1$}-+-{empty:->w2$}-+-{empty:->w3$}-+-{empty:->w4$}-+",
                empty = "",
            )
        };
        let barln = |f: &mut Formatter| {
            bar(f)?;
            writeln!(f)
        };
        let line = |f: &mut Formatter, num, name, vp, price, stock| {
            writeln!(
                f,
                "| {num:>w0$} | {name:<w1$} | {vp:>w2$} | {price:>w3$} | {stock:>w4$} |"
            )
        };
        let line_ = |f: &mut Formatter, num, name, vp, price, stock| {
            writeln!(
                f,
                "| {num:>w0$} | {name:<w1$} | {vp:>w2$} | {price:>w3$} | {stock:>w4$} |"
            )
        };

        barln(f)?;
        line(f, "No.", "Name", VP_DISPLAY, "Price", "Stock")?;
        barln(f)?;
        for (i, (card, n)) in self.slots.iter().enumerate() {
            line_(f, i, separate_uppers(card), card.vp(), card.price(), n)?;
        }
        bar(f)?;

        Ok(())
    }
}

/// # Example
///
/// ```
/// assert_eq!(separate_uppers("HelloWorld"), "Hello World");
/// ```
fn separate_uppers<T: Display>(upper_camel: &T) -> String {
    upper_camel
        .to_string()
        .chars()
        .map(|c| {
            if c.is_uppercase() {
                format!(" {c}")
            } else {
                c.into()
            }
        })
        .collect::<String>()
        .trim()
        .into()
}
