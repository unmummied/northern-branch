pub mod building;
pub mod product1;
pub mod product2;
pub mod resource;

use crate::{
    action::produce_or_barter::StockInt,
    state::{
        PopulationInt,
        queue::{
            ERR_TOO_FEW_PLAYERS, ERR_TOO_MUCH_PLAYERS, MAXIMUM_PLAYERS_LEN, MINIMUM_PLAYERS_LEN,
        },
    },
};
use building::{Building, basic::BasicBuilding, normal::NormalBuilding, special::SpecialBuilding};
use product1::Product1;
use product2::Product2;
use resource::Resource;
use std::{
    collections::BTreeMap,
    fmt::{self, Display, Formatter},
};
use strum::{EnumIs, EnumIter, IntoEnumIterator};

pub const VP_DISPLAY: &str = "VP";
const ERR_EMPTY_ENUM: &str = "empty enum...";

pub type PriceInt = i8;
pub type VPInt = u8;
pub trait Value: Sized {
    fn price(&self) -> PriceInt;
    fn vp(&self) -> VPInt;
}

pub trait Quantity {
    fn bound_check(population: PopulationInt) -> Result<(), &'static str> {
        if population < MINIMUM_PLAYERS_LEN {
            return Err(ERR_TOO_FEW_PLAYERS);
        }
        if MAXIMUM_PLAYERS_LEN < population {
            return Err(ERR_TOO_MUCH_PLAYERS);
        }
        Ok(())
    }
    fn quantity(&self, population: PopulationInt) -> Result<StockInt, &'static str>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIs, EnumIter)]
pub enum Card {
    Resource(Resource),
    Product1(Product1),
    Product2(Product2),
    Building(Building),
    OneVP,
}

impl Card {
    pub fn deck(population: PopulationInt) -> Result<BTreeMap<Self, StockInt>, &'static str> {
        let resources = Resource::iter();
        let product1s = Product1::iter();
        let product2s = Product2::iter();
        let buildings = Building::all_iter();

        resources
            .map(Self::from)
            .chain(product1s.map(Into::<_>::into))
            .chain(product2s.map(Into::<_>::into))
            .chain(buildings.map(Into::<_>::into))
            .map(|card| {
                let n = card.quantity(population)?;
                Ok((card, n))
            })
            .collect()
    }
}

impl Value for Card {
    fn price(&self) -> PriceInt {
        match self {
            Self::Resource(x) => x.price(),
            Self::Product1(x) => x.price(),
            Self::Product2(x) => x.price(),
            Self::Building(x) => x.price(),
            Self::OneVP => 0,
        }
    }

    fn vp(&self) -> VPInt {
        match self {
            Self::Resource(x) => x.vp(),
            Self::Product1(x) => x.vp(),
            Self::Product2(x) => x.vp(),
            Self::Building(x) => x.vp(),
            Self::OneVP => 1,
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
            Self::OneVP => Ok(11),
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
                Self::OneVP => Self::OneVP,
            })
            .expect(ERR_EMPTY_ENUM)
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Resource(resource) => resource.fmt(f),
            Self::Product1(product1) => product1.fmt(f),
            Self::Product2(product2) => product2.fmt(f),
            Self::Building(building) => building.fmt(f),
            Self::OneVP => write!(f, "{VP_DISPLAY}"),
        }
    }
}
