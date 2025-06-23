use super::super::{Value, ValueInt, VictInt};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BasicBuilding {
    Smelter,
    GlassFactory,
    PigFarm,
    CompostFarm,
    PoultryFarm,
    CementFactory,
    FuelFactory,
    Sawmill,
}

impl Value for BasicBuilding {
    fn value(&self) -> ValueInt {
        match self {
            Self::Smelter
            | Self::GlassFactory
            | Self::PigFarm
            | Self::CompostFarm
            | Self::PoultryFarm
            | Self::CementFactory => 6,
            Self::FuelFactory => 7,
            Self::Sawmill => 8,
        }
    }

    fn victory_points(&self) -> VictInt {
        1
    }
}
