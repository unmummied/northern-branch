use super::{ERR_EMPTY_ENUM, Quantity, Value, PriceInt, VPInt};
use crate::{action::produce_or_barter::StockInt, state::PopulationInt};
use strum::{Display, EnumIter, IntoEnumIterator};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Display, EnumIter)]
pub enum Product1 {
    Fuel,
    Cement,
    Pig,
    Timber,
    Bronze,
    Compost,
    Mushroom,
    Chicken,
    Glass,
}

impl Value for Product1 {
    fn price(&self) -> PriceInt {
        match self {
            Self::Fuel | Self::Cement | Self::Pig | Self::Timber | Self::Bronze => 3,
            Self::Compost | Self::Mushroom => 4,
            Self::Chicken | Self::Glass => 5,
        }
    }

    fn vp(&self) -> VPInt {
        0
    }
}

impl Quantity for Product1 {
    fn quantity(&self, population: PopulationInt) -> Result<StockInt, &'static str> {
        Self::bound_check(population)?;
        Ok(match self {
            Self::Mushroom => 1,
            _ => 2,
        })
    }
}

impl Default for Product1 {
    fn default() -> Self {
        Self::iter().next().expect(ERR_EMPTY_ENUM)
    }
}
