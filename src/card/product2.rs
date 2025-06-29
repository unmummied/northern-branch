use super::{EMPTY_ENUM_ERR, Quantity, Value, ValueInt, VictInt};
use crate::{action::produce_or_barter::StockInt, state::PopulationInt};
use strum::{Display, EnumIter, IntoEnumIterator};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Display, EnumIter)]
pub enum Product2 {
    Egg,
    Concrete,
    Furniture,
    Vegetables,
    Down,
    Sausage,
    Beer,
    Broadax,
    Mirror,
}

impl Value for Product2 {
    fn value(&self) -> ValueInt {
        match self {
            Self::Egg => 3,
            Self::Concrete | Self::Furniture => 6,
            Self::Vegetables => 7,
            Self::Down | Self::Sausage | Self::Beer => 8,
            Self::Broadax => 9,
            Self::Mirror => 13,
        }
    }

    fn victory_points(&self) -> VictInt {
        0
    }
}

impl Quantity for Product2 {
    fn quantity(&self, population: PopulationInt) -> Result<StockInt, &'static str> {
        Self::bound_check(population)?;
        Ok(1)
    }
}

impl Default for Product2 {
    fn default() -> Self {
        Self::iter().next().expect(EMPTY_ENUM_ERR)
    }
}
