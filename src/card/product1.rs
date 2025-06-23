use super::{Value, ValueInt, VictInt};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
}
