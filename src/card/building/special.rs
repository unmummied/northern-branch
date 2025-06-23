use super::super::{Value, ValueInt, VictInt};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
