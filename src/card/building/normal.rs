use super::super::{Value, ValueInt, VictInt};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
}
