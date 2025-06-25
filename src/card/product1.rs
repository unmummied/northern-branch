use super::{CardInfo, EMPTY_ENUM_ERR, ValueInt, VictInt};
use crate::action::produce_or_barter::StockInt;
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
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

impl CardInfo for Product1 {
    fn value(&self) -> ValueInt {
        match self {
            Self::Fuel | Self::Cement | Self::Pig | Self::Timber | Self::Bronze => 3,
            Self::Compost | Self::Mushroom => 4,
            Self::Chicken | Self::Glass => 5,
        }
    }

    fn victory_points(&self) -> VictInt {
        0
    }

    fn total_n(&self, _: usize) -> StockInt {
        match self {
            Self::Mushroom => 1,
            _ => 2,
        }
    }
}

impl Default for Product1 {
    fn default() -> Self {
        Self::iter().next().expect(EMPTY_ENUM_ERR)
    }
}
