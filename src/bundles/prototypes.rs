use crate::bundles::buildables::torch::WoodenTorch;
use crate::bundles::buildables::water_well::WaterWell;
use crate::bundles::buildables::wooden_wall::WoodenWall;
use crate::bundles::plants::oak_tree::OakTree;
use crate::bundles::plants::potato_plant::{PotatoPlant, PotatoPlantSeed};
use crate::bundles::resources::lumber::Lumber;
use crate::bundles::resources::water::Water;
use crate::bundles::settler::Settler;
use crate::bundles::soil::dirt::Dirt;
use crate::bundles::{ItemId, Prototypes};
use crate::features::misc_components::Prototype;
use bevy::prelude::{Commands, ResMut};
use crate::bundles::plants::barren_tree::BarrenTree;
use crate::bundles::plants::maple_tree::MapleTree;
use crate::bundles::plants::pine_tree::PineTree;

macro_rules! create_prototypes {
    ( $commands:expr,$prototypes:expr,$( ($item_id:expr, $component:expr) ),*, ) => {
        {
            $(
                $prototypes.0.insert($item_id, $commands.spawn(($component, Prototype)).id());
            )*
        }
    };
}

pub fn setup_prototypes(
    mut prototypes: ResMut<Prototypes>,
    mut commands: Commands,
) {
    create_prototypes!(
        commands,
        prototypes,
        (ItemId::WoodenTorch, WoodenTorch),
        (ItemId::WoodenWall, WoodenWall),
        (ItemId::Settler, Settler::default()),
        (ItemId::Lumber, Lumber),
        (ItemId::OakTree, OakTree),
        (ItemId::MapleTree, MapleTree),
        (ItemId::PineTree, PineTree),
        (ItemId::BarrenTree, BarrenTree),
        (ItemId::Water, Water),
        (ItemId::Dirt, Dirt),
        (ItemId::PotatoPlant, PotatoPlant),
        (ItemId::PotatoPlantSeed, PotatoPlantSeed),
        (ItemId::WaterWell, WaterWell),
    );
}
