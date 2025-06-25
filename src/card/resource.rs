use crate::action::produce_or_barter::StockInt;
use strum::EnumIter;

use super::{Value, ValueInt, VictInt};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum Resource {
    Dung,
    Clay,
    Barley,
    Wood,
    Ore,
}

impl Value for Resource {
    fn value(&self) -> ValueInt {
        match self {
            Self::Dung => -1,
            _ => 1,
        }
    }

    fn victory_points(&self) -> VictInt {
        0
    }

    fn total_n(&self, member: usize) -> StockInt {
        let idx = member.saturating_sub(2);
        match self {
            Self::Dung => [4, 5, 6][idx],
            Self::Clay => [3, 3, 4][idx],
            Self::Barley | Self::Wood => [2, 3, 3][idx],
            Self::Ore => [2, 2, 3][idx],
        }
    }
}
