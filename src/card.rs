pub mod building;
pub mod product1;
pub mod product2;
pub mod resource;

use building::{Building, basic::BasicBuilding, normal::NormalBuilding, special::SpecialBuilding};
use product1::Product1;
use product2::Product2;
use resource::Resource;

pub type ValueInt = i8;
type VictInt = u8;
pub trait Value: Sized {
    fn value(&self) -> ValueInt;
    fn victory_points(&self) -> VictInt;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Card {
    Resource(Resource),
    Product1(Product1),
    Product2(Product2),
    Building(Building),
    VictoryPoint,
}

impl Value for Card {
    fn value(&self) -> ValueInt {
        match self {
            Self::Resource(x) => x.value(),
            Self::Product1(x) => x.value(),
            Self::Product2(x) => x.value(),
            Self::Building(x) => x.value(),
            Self::VictoryPoint => 0,
        }
    }

    fn victory_points(&self) -> VictInt {
        match self {
            Self::Resource(x) => x.victory_points(),
            Self::Product1(x) => x.victory_points(),
            Self::Product2(x) => x.victory_points(),
            Self::Building(x) => x.victory_points(),
            Self::VictoryPoint => 1,
        }
    }
}

impl From<Resource> for Card {
    fn from(resource: Resource) -> Self {
        Self::Resource(resource)
    }
}
impl From<Product1> for Card {
    fn from(product1: Product1) -> Self {
        Self::Product1(product1)
    }
}
impl From<Product2> for Card {
    fn from(product2: Product2) -> Self {
        Self::Product2(product2)
    }
}
impl From<Building> for Card {
    fn from(building: Building) -> Self {
        Self::Building(building)
    }
}
impl From<BasicBuilding> for Card {
    fn from(basic: BasicBuilding) -> Self {
        Building::from(basic).into()
    }
}
impl From<NormalBuilding> for Card {
    fn from(normal: NormalBuilding) -> Self {
        Building::from(normal).into()
    }
}
impl From<SpecialBuilding> for Card {
    fn from(special: SpecialBuilding) -> Self {
        Building::from(special).into()
    }
}
