use super::{Value, ValueInt, VictInt};
use crate::action::produce_or_barter::StockInt;
use strum::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
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

    fn total_n(&self, _: usize) -> StockInt {
        1
    }
}
