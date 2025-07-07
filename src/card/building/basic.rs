use super::super::{Value, ValueInt, VictInt};
use crate::{
    action::produce_or_barter::StockInt,
    card::{ERR_EMPTY_ENUM, Quantity},
    state::PopulationInt,
};
use rand::{Rng, seq::IteratorRandom};
use strum::{Display, EnumIter, IntoEnumIterator};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Display, EnumIter)]
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

impl BasicBuilding {
    pub fn chosen_basics<R: Rng>(
        rng: &mut R,
        population: PopulationInt,
    ) -> impl Iterator<Item = Self> {
        Self::iter()
            .choose_multiple(rng, population + 1)
            .into_iter()
    }
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
        Self::iter().next().expect(ERR_EMPTY_ENUM)
    }
}
