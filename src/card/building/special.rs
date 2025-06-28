use super::super::{Value, ValueInt, VictInt};
use crate::{
    action::produce_or_barter::StockInt,
    card::{EMPTY_ENUM_ERR, Quantity},
    state::PopulationInt,
};
use strum::{Display, EnumIter, IntoEnumIterator};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter, Display)]
pub enum SpecialBuilding {
    Exchange,
    Realtor,
    Market,
    TradingHouse,
}

impl Value for SpecialBuilding {
    fn value(&self) -> ValueInt {
        match self {
            Self::Exchange => 6,
            Self::Realtor | Self::Market => 10,
            Self::TradingHouse => 23,
        }
    }

    fn victory_points(&self) -> VictInt {
        match self {
            Self::Exchange => 1,
            Self::Realtor | Self::Market => 2,
            Self::TradingHouse => 4,
        }
    }
}

impl Quantity for SpecialBuilding {
    fn quantity(&self, population: PopulationInt) -> Result<StockInt, &'static str> {
        Self::bound_check(population)?;
        Ok(1)
    }
}

impl Default for SpecialBuilding {
    fn default() -> Self {
        Self::iter().next().expect(EMPTY_ENUM_ERR)
    }
}
