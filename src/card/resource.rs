use super::{ERR_EMPTY_ENUM, Quantity, Value, PriceInt, VPInt};
use crate::{action::produce_or_barter::StockInt, state::PopulationInt};
use strum::{Display, EnumIter, IntoEnumIterator};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Display, EnumIter)]
pub enum Resource {
    Dung,
    Clay,
    Barley,
    Wood,
    Ore,
}

impl Value for Resource {
    fn price(&self) -> PriceInt {
        match self {
            Self::Dung => -1,
            _ => 1,
        }
    }

    fn vp(&self) -> VPInt {
        0
    }
}

impl Quantity for Resource {
    fn quantity(&self, population: PopulationInt) -> Result<StockInt, &'static str> {
        Self::bound_check(population)?;
        let idx = population.saturating_sub(2);
        Ok(match self {
            Self::Dung => [4, 5, 6][idx],
            Self::Clay => [3, 3, 4][idx],
            Self::Barley | Self::Wood => [2, 3, 3][idx],
            Self::Ore => [2, 2, 3][idx],
        })
    }
}

impl Default for Resource {
    fn default() -> Self {
        Self::iter().next().expect(ERR_EMPTY_ENUM)
    }
}
