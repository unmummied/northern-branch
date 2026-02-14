use crate::{
    action::produce_or_barter::{
        StockInt,
        barter::Barter,
        produce::{
            Recip,
            recip::{RecipBy, dst::Dst, src::Src},
        },
    },
    card::{Card, Value, VPInt, VP_DISPLAY, building::Building},
};
use std::{
    collections::BTreeMap,
    fmt::{self, Display, Formatter},
};

const ERR_UNKNOWN_RECIP: &str = "unknown recip...";
const ERR_INSUFFICIENT_SRC: &str = "src is insufficient...";
const ERR_INVALID_BARTER: &str = "invalid barter...";
pub const ERR_FAILED_FORCE_INTO_GIVE_N_TAKE_N: &str = "`force_into_give_n_take_n` is failed...";
const MAX_CARDS_LEN: StockInt = 7;
const ERR_CARDS_LEN_IS_TOO_LONG: &str = "cards len is too long...";

#[derive(Debug, Default, Clone)]
pub struct Inventory {
    pub cards: BTreeMap<Card, StockInt>,
    pub buildings: BTreeMap<Building, StockInt>, // if building is unique, this is redundant.
    pub vp: VPInt,
}

impl Inventory {
    fn cards_len(&self) -> StockInt {
        self.cards.values().sum()
    }
    fn buildings_len(&self) -> StockInt {
        self.buildings.values().sum()
    }
    fn total_vps(&self) -> VPInt {
        self.vp
            + self
                .buildings
                .iter()
                .map(|(building, n)| building.vp() * (*n as VPInt))
                .sum::<VPInt>()
    }

    fn is_cards_len_valid(&self) -> bool {
        self.cards_len() <= MAX_CARDS_LEN
    }

    fn is_subset(&self, superset: &Self) -> bool {
        is_subset(&self.cards, &superset.cards)
            && is_subset(&self.buildings, &superset.buildings)
            && self.vp <= superset.vp
    }

    fn union(&self, other: &Self) -> Self {
        Self {
            cards: union(&self.cards, &other.cards),
            buildings: union(&self.buildings, &other.buildings),
            vp: self.vp + other.vp,
        }
    }

    fn difference(&self, other: &Self) -> Self {
        Self {
            cards: difference(&self.cards, &other.cards),
            buildings: difference(&self.buildings, &other.buildings),
            vp: self.vp.saturating_sub(other.vp),
        }
    }

    pub fn try_produce_clone(
        &self,
        recip: &Recip,
        book: &RecipBy<Src, Dst>,
    ) -> Result<Self, &'static str> {
        if !recip.is_in(book) {
            return Err(ERR_UNKNOWN_RECIP);
        }
        let src = Into::<Self>::into(recip.src.clone());
        let consumed = recip.src.clone().consume_cards().into();
        if !src.is_subset(self) {
            return Err(ERR_INSUFFICIENT_SRC);
        }
        let dst = recip.dst.clone().into();
        let res = self.difference(&consumed).union(&dst);
        if !res.is_cards_len_valid() {
            return Err(ERR_CARDS_LEN_IS_TOO_LONG);
        }
        Ok(res)
    }

    pub fn try_barter_clone(&self, barter: &Barter) -> Result<Self, &'static str> {
        if !barter.is_affordable() {
            return Err(ERR_INVALID_BARTER);
        }
        let Barter::GiveNTakeN { give, take } = barter.clone().force_into_give_n_take_n() else {
            return Err(ERR_FAILED_FORCE_INTO_GIVE_N_TAKE_N);
        };
        let res = self.difference(&give.into()).union(&take.into());
        if !res.is_cards_len_valid() {
            return Err(ERR_CARDS_LEN_IS_TOO_LONG);
        }
        Ok(res)
    }
}

impl From<Src> for Inventory {
    fn from(src: Src) -> Self {
        union(&src.clone().consume_cards(), &src.retain_cards()).into()
    }
}
impl From<Dst> for Inventory {
    fn from(dst: Dst) -> Self {
        dst.dst.into()
    }
}
impl From<BTreeMap<Card, StockInt>> for Inventory {
    fn from(map: BTreeMap<Card, StockInt>) -> Self {
        let mut cards = BTreeMap::new();
        let mut buildings = BTreeMap::new();
        let mut vp = 0;
        for (card, n) in map {
            match card {
                Card::Building(building) => {
                    buildings.insert(building, n);
                }
                Card::OneVP => {
                    vp += n;
                }
                _ => {
                    cards.insert(card, n);
                }
            }
        }
        Self {
            cards,
            buildings,
            vp,
        }
    }
}

fn is_subset<K: Ord>(subset: &BTreeMap<K, StockInt>, superset: &BTreeMap<K, StockInt>) -> bool {
    subset
        .iter()
        .all(|(key, value)| *value <= superset.get(key).copied().unwrap_or_default())
}

fn union<K: Clone + Ord>(
    lhs: &BTreeMap<K, StockInt>,
    rhs: &BTreeMap<K, StockInt>,
) -> BTreeMap<K, StockInt> {
    lhs.keys()
        .chain(rhs.keys())
        .map(|key| {
            (
                key.clone(),
                lhs.get(key).copied().unwrap_or_default()
                    + rhs.get(key).copied().unwrap_or_default(),
            )
        })
        .collect()
}

fn difference<K: Clone + Ord>(
    lhs: &BTreeMap<K, StockInt>,
    rhs: &BTreeMap<K, StockInt>,
) -> BTreeMap<K, StockInt> {
    lhs.iter()
        .filter_map(|(key, value)| {
            let diff = value.saturating_sub(rhs.get(key).copied().unwrap_or_default());
            (0 < diff).then(|| (key.clone(), diff))
        })
        .collect()
}

impl Display for Inventory {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(
            f,
            "{} cards, {} buildings, and {} {VP_DISPLAY}s.",
            self.cards_len(),
            self.buildings_len(),
            self.total_vps()
        )?;
        writeln!(
            f,
            "    cards: {:?}",
            self.cards
                .iter()
                .map(|(card, n)| format!("({card}, {n})"))
                .collect::<Vec<_>>()
        )?;
        write!(
            f,
            "buildings: {:?}",
            self.buildings
                .iter()
                .map(|(building, n)| format!("({building}, {n})"))
                .collect::<Vec<_>>()
        )?;
        Ok(())
    }
}
