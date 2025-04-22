use crate::bundles::buildables::torch::WoodenTorch;
use crate::bundles::buildables::water_well::WaterWell;
use crate::bundles::buildables::wooden_wall::WoodenWall;
use crate::bundles::plants::oak_tree::OakTree;
use crate::bundles::plants::potato_plant::{PotatoPlant, PotatoPlantSeed};
use crate::bundles::resources::lumber::Lumber;
use crate::bundles::resources::water::Water;
use crate::bundles::settler::Settler;
use crate::bundles::soil::dirt::Dirt;
use crate::bundles::{ItemId, ItemSpawners, Prototypes};
use crate::features::misc_components::Prototype;
use bevy::prelude::{Commands, ResMut};
use crate::bundles::plants::barren_tree::BarrenTree;
use crate::bundles::plants::maple_tree::MapleTree;
use crate::bundles::plants::pine_tree::PineTree;

macro_rules! create_spawners_and_prototypes {
    ( $commands:expr,$spawners:expr,$prototypes:expr,$( ($item_id:expr, $component:expr) ),*, ) => {
        {
            $(
                $prototypes.0.insert($item_id, $commands.spawn(($component, Prototype)).id());
                $spawners.insert($item_id, |commands| {
                    commands.spawn(($component,)).id()
                });
            )*
        }
    };
}

pub fn setup_spawners_and_prototypes(
    mut prototypes: ResMut<Prototypes>,
    mut spawners: ResMut<ItemSpawners>,
    mut commands: Commands,
) {
    create_spawners_and_prototypes!(
        commands,
        spawners,
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
