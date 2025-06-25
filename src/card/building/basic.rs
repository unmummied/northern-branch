use super::super::{CardInfo, ValueInt, VictInt};
use crate::{action::produce_or_barter::StockInt, card::EMPTY_ENUM_ERR};
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum BasicBuilding {
    Smelter,
    GlassFactory,
    PigFarm,
    CompostFarm,
    PoultryFarm,
    CementFactory,
    FuelFactory,
    Sawmill,
}

impl CardInfo for BasicBuilding {
    fn value(&self) -> ValueInt {
        match self {
            Self::Smelter
            | Self::GlassFactory
            | Self::PigFarm
            | Self::CompostFarm
            | Self::PoultryFarm
            | Self::CementFactory => 6,
            Self::FuelFactory => 7,
            Self::Sawmill => 8,
        }
    }

    fn victory_points(&self) -> VictInt {
        1
    }

    fn total_n(&self, _: usize) -> StockInt {
        1
    }
}

impl Default for BasicBuilding {
    fn default() -> Self {
        Self::iter().next().expect(EMPTY_ENUM_ERR)
    }
}
