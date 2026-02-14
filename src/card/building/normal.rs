use super::super::{Value, PriceInt, VPInt};
use crate::{
    action::produce_or_barter::StockInt,
    card::{ERR_EMPTY_ENUM, Quantity},
    state::PopulationInt,
};
use strum::{Display, EnumIter, IntoEnumIterator};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Display, EnumIter)]
pub enum NormalBuilding {
    FurnitureFactory,
    DownFarm,
    BuildingMaterialFactory,
    EggFarm,
    MushroomGarden,
    Bar,
    MirrorFactory,
    Brewery,
    VegetablesFarm,
    SausageFactory,
    CompostMaker,
    GlassCraftWorkshop,
    Greengrocer,
    LiquorStore,
    BuildingSuppliesShop,
    BeddingShop,
    MirrorShop,
    Diner,
}

impl Value for NormalBuilding {
    fn price(&self) -> PriceInt {
        match self {
            Self::FurnitureFactory
            | Self::DownFarm
            | Self::BuildingMaterialFactory
            | Self::EggFarm
            | Self::MushroomGarden
            | Self::Bar
            | Self::MirrorFactory
            | Self::Brewery
            | Self::VegetablesFarm => 6,
            Self::SausageFactory => 7,
            Self::CompostMaker => 8,
            Self::GlassCraftWorkshop
            | Self::Greengrocer
            | Self::LiquorStore
            | Self::BuildingSuppliesShop
            | Self::BeddingShop
            | Self::MirrorShop => 10,
            Self::Diner => 12,
        }
    }

    fn vp(&self) -> VPInt {
        match self {
            Self::FurnitureFactory
            | Self::DownFarm
            | Self::BuildingMaterialFactory
            | Self::EggFarm
            | Self::MushroomGarden
            | Self::Bar
            | Self::MirrorFactory
            | Self::Brewery
            | Self::VegetablesFarm
            | Self::SausageFactory
            | Self::CompostMaker
            | Self::BuildingSuppliesShop => 1,
            _ => 2,
        }
    }
}

impl Quantity for NormalBuilding {
    fn quantity(&self, population: PopulationInt) -> Result<StockInt, &'static str> {
        Self::bound_check(population)?;
        Ok(1)
    }
}

impl Default for NormalBuilding {
    fn default() -> Self {
        Self::iter().next().expect(ERR_EMPTY_ENUM)
    }
}
