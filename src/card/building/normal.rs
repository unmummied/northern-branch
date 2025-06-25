use super::super::{CardInfo, ValueInt, VictInt};
use crate::{action::produce_or_barter::StockInt, card::EMPTY_ENUM_ERR};
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
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

impl CardInfo for NormalBuilding {
    fn value(&self) -> ValueInt {
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

    fn victory_points(&self) -> VictInt {
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

    fn total_n(&self, _: usize) -> StockInt {
        1
    }
}

impl Default for NormalBuilding {
    fn default() -> Self {
        Self::iter().next().expect(EMPTY_ENUM_ERR)
    }
}
