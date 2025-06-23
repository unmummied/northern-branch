pub mod basic;
pub mod normal;
pub mod special;

use super::{Value, ValueInt, VictInt};
use basic::BasicBuilding;
use normal::NormalBuilding;
use special::SpecialBuilding;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Building {
    Basic(BasicBuilding),
    Normal(NormalBuilding),
    Special(SpecialBuilding),
}

impl Value for Building {
    fn value(&self) -> ValueInt {
        match self {
            Self::Basic(b) => b.value(),
            Self::Normal(b) => b.value(),
            Self::Special(b) => b.value(),
        }
    }

    fn victory_points(&self) -> VictInt {
        match self {
            Self::Basic(b) => b.victory_points(),
            Self::Normal(b) => b.victory_points(),
            Self::Special(b) => b.victory_points(),
        }
    }
}

impl From<BasicBuilding> for Building {
    fn from(basic: BasicBuilding) -> Self {
        Self::Basic(basic)
    }
}
impl From<NormalBuilding> for Building {
    fn from(normal: NormalBuilding) -> Self {
        Self::Normal(normal)
    }
}
impl From<SpecialBuilding> for Building {
    fn from(special: SpecialBuilding) -> Self {
        Self::Special(special)
    }
}
