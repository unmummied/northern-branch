use super::{Value, ValueInt, VictInt};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
}
