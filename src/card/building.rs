pub mod basic;
pub mod normal;
pub mod special;

use super::{EMPTY_ENUM_ERR, Quantity, Value, ValueInt, VictInt};
use crate::{action::produce_or_barter::StockInt, state::PopulationInt};
use basic::BasicBuilding;
use normal::NormalBuilding;
use special::SpecialBuilding;
use std::fmt::{self, Display, Formatter};
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum Building {
    Basic(BasicBuilding),
    Normal(NormalBuilding),
    Special(SpecialBuilding),
}

impl Building {
    pub fn all_iter() -> impl Iterator<Item = Self> {
        let basics = BasicBuilding::iter();
        let normals = NormalBuilding::iter();
        let specials = SpecialBuilding::iter();
        basics
            .map(Self::from)
            .chain(normals.map(Into::<_>::into))
            .chain(specials.map(Into::<_>::into))
    }
}

impl Value for Building {
    fn value(&self) -> ValueInt {
        match self {
            Self::Basic(basic) => basic.value(),
            Self::Normal(normal) => normal.value(),
            Self::Special(special) => special.value(),
        }
    }

    fn victory_points(&self) -> VictInt {
        match self {
            Self::Basic(basic) => basic.victory_points(),
            Self::Normal(normal) => normal.victory_points(),
            Self::Special(special) => special.victory_points(),
        }
    }
}

impl Quantity for Building {
    fn quantity(&self, population: PopulationInt) -> Result<StockInt, &'static str> {
        match self {
            Self::Basic(basic) => basic.quantity(population),
            Self::Normal(normal) => normal.quantity(population),
            Self::Special(special) => special.quantity(population),
        }
    }
}

impl From<BasicBuilding> for Building {
    fn from(basic: BasicBuilding) -> Self {
        Self::Basic(basic)
    }
}
impl From<NormalBuilding> for Building {
    fn from(normal: NormalBuilding) -> Self {
        Self::Normal(normal)
    }
}
impl From<SpecialBuilding> for Building {
    fn from(special: SpecialBuilding) -> Self {
        Self::Special(special)
    }
}

impl Default for Building {
    fn default() -> Self {
        Self::iter()
            .next()
            .map(|building| match building {
                Self::Basic(_) => Self::Basic(BasicBuilding::default()),
                Self::Normal(_) => Self::Normal(NormalBuilding::default()),
                Self::Special(_) => Self::Special(SpecialBuilding::default()),
            })
            .expect(EMPTY_ENUM_ERR)
    }
}

impl Display for Building {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Basic(basic) => basic.fmt(f),
            Self::Normal(normal) => normal.fmt(f),
            Self::Special(special) => special.fmt(f),
        }
    }
}
