pub mod basic;
pub mod normal;
pub mod special;

use super::{CardInfo, EMPTY_ENUM_ERR, ValueInt, VictInt};
use crate::action::produce_or_barter::StockInt;
use basic::BasicBuilding;
use normal::NormalBuilding;
use special::SpecialBuilding;
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum Building {
    Basic(BasicBuilding),
    Normal(NormalBuilding),
    Special(SpecialBuilding),
}

impl CardInfo for Building {
    fn value(&self) -> ValueInt {
        match self {
            Self::Basic(b) => b.value(),
            Self::Normal(b) => b.value(),
            Self::Special(b) => b.value(),
        }
    }

    fn victory_points(&self) -> VictInt {
        match self {
            Self::Basic(b) => b.victory_points(),
            Self::Normal(b) => b.victory_points(),
            Self::Special(b) => b.victory_points(),
        }
    }

    fn total_n(&self, member: usize) -> StockInt {
        match self {
            Self::Basic(basic_building) => basic_building.total_n(member),
            Self::Normal(normal_building) => normal_building.total_n(member),
            Self::Special(special_building) => special_building.total_n(member),
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
