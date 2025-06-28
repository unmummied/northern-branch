pub mod building;
pub mod product1;
pub mod product2;
pub mod resource;

use crate::{action::produce_or_barter::StockInt, state::PopulationInt};
use building::{Building, basic::BasicBuilding, normal::NormalBuilding, special::SpecialBuilding};
use product1::Product1;
use product2::Product2;
use resource::Resource;
use std::{
    collections::BTreeMap,
    fmt::{self, Display, Formatter},
};
use strum::{EnumIter, IntoEnumIterator};

const VICTORY_POINT_DISPLAY: &str = "VP";
const EMPTY_ENUM_ERR: &str = "empty enum...";

pub type ValueInt = i8;
type VictInt = u8;
pub trait Value: Sized {
    fn value(&self) -> ValueInt;
    fn victory_points(&self) -> VictInt;
}

pub trait Quantity {
    const MINIMUM_PLAYERS_LEN: PopulationInt = 2;
    const MAXIMUM_PLAYERS_LEN: PopulationInt = 4;
    const TOO_FEW_PLAYERS_ERR: &str = "too few players...";
    const TOO_MUCH_PLAYERS_ERR: &str = "too mush players...";
    fn bound_check(population: PopulationInt) -> Result<(), &'static str> {
        if population < Self::MINIMUM_PLAYERS_LEN {
            return Err(Self::TOO_FEW_PLAYERS_ERR);
        }
        if Self::MAXIMUM_PLAYERS_LEN < population {
            return Err(Self::TOO_MUCH_PLAYERS_ERR);
        }
        Ok(())
    }
    fn quantity(&self, population: PopulationInt) -> Result<StockInt, &'static str>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum Card {
    Resource(Resource),
    Product1(Product1),
    Product2(Product2),
    Building(Building),
    VictoryPoint,
}

impl Card {
    pub fn deck(population: PopulationInt) -> Result<BTreeMap<Self, StockInt>, &'static str> {
        let resources = Resource::iter();
        let product1 = Product1::iter();
        let product2 = Product2::iter();
        let basic_buildings = BasicBuilding::iter();
        let normal_buildings = NormalBuilding::iter();
        let special_buildings = SpecialBuilding::iter();

        resources
            .map(Self::from)
            .chain(product1.map(Into::<_>::into))
            .chain(product2.map(Into::<_>::into))
            .chain(basic_buildings.map(Into::<_>::into))
            .chain(normal_buildings.map(Into::<_>::into))
            .chain(special_buildings.map(Into::<_>::into))
            .map(|card| {
                let n = card.quantity(population)?;
                Ok((card, n))
            })
            .collect()
    }
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

impl Quantity for Card {
    fn quantity(&self, population: PopulationInt) -> Result<StockInt, &'static str> {
        Self::bound_check(population)?;
        match self {
            Self::Resource(resource) => resource.quantity(population),
            Self::Product1(product1) => product1.quantity(population),
            Self::Product2(product2) => product2.quantity(population),
            Self::Building(building) => building.quantity(population),
            Self::VictoryPoint => Ok(11),
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

impl Default for Card {
    fn default() -> Self {
        Self::iter()
            .next()
            .map(|card| match card {
                Self::Resource(_) => Self::Resource(Resource::default()),
                Self::Product1(_) => Self::Product1(Product1::default()),
                Self::Product2(_) => Self::Product2(Product2::default()),
                Self::Building(_) => Self::Building(Building::default()),
                Self::VictoryPoint => Self::VictoryPoint,
            })
            .expect(EMPTY_ENUM_ERR)
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Resource(resource) => resource.fmt(f),
            Self::Product1(product1) => product1.fmt(f),
            Self::Product2(product2) => product2.fmt(f),
            Self::Building(building) => building.fmt(f),
            Self::VictoryPoint => write!(f, "{VICTORY_POINT_DISPLAY}"),
        }
    }
}
