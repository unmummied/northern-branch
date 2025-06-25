use super::super::{CardInfo, ValueInt, VictInt};
use crate::{action::produce_or_barter::StockInt, card::EMPTY_ENUM_ERR};
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum SpecialBuilding {
    Exchange,
    Realtor,
    Market,
    TradingHouse,
}

impl CardInfo for SpecialBuilding {
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

    fn total_n(&self, _: usize) -> StockInt {
        1
    }
}

impl Default for SpecialBuilding {
    fn default() -> Self {
        Self::iter().next().expect(EMPTY_ENUM_ERR)
    }
}
