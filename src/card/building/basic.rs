use super::super::{Value, ValueInt, VictInt};
use crate::{
    action::produce_or_barter::StockInt,
    card::{EMPTY_ENUM_ERR, Quantity},
    state::PopulationInt,
};
use strum::{Display, EnumIter, IntoEnumIterator};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter, Display)]
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

impl Value for BasicBuilding {
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
}

impl Quantity for BasicBuilding {
    fn quantity(&self, population: PopulationInt) -> Result<StockInt, &'static str> {
        Self::bound_check(population)?;
        Ok(1)
    }
}

impl Default for BasicBuilding {
    fn default() -> Self {
        Self::iter().next().expect(EMPTY_ENUM_ERR)
    }
}
