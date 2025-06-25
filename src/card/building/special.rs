use super::super::{Value, ValueInt, VictInt};
use crate::action::produce_or_barter::StockInt;
use strum::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
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

    fn total_n(&self, _: usize) -> StockInt {
        1
    }
}
