use crate::features::misc_components::ItemAmount;
use crate::bundles::buildables::Buildable;
use crate::features::health::Health;
use crate::features::item_drop::SingleItemDrop;
use crate::bundles::{Id, ItemId, ResourceItem};
use crate::features::assets::GltfAssetId;
use crate::features::misc_components::gltf_asset::GltfData;
use crate::features::plants::Plant;
use crate::features::item_drop::ItemDrop;
use bevy::prelude::*;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
#[require(
    Id(|| Id(ItemId::PotatoPlant)),
    Name(|| "Potato Plant"),
    GltfData(|| GltfData {
        asset_id: GltfAssetId::PotatoPlant,
        scene_name: None
    }),
    Plant(|| Plant {
        growth_stages: 4,
        growth_requirements: vec![
            ItemAmount {
                item_id: ItemId::Water,
                amount: 1
            }
        ],
        ..Default::default()
    }),
    Buildable(|| Buildable {
        item_requirements: vec![
            ItemAmount {
                item_id: ItemId::PotatoPlantSeed,
                amount: 1,
            }
        ],
        ..Default::default()
    }),
    Health(|| Health::new(1.0)),
    ItemDrop(|| ItemDrop {
        item_drops: vec![
            SingleItemDrop { item_id: ItemId::Potato, chance: 1.0 },
            SingleItemDrop { item_id: ItemId::Potato, chance: 0.5 },
            SingleItemDrop { item_id: ItemId::Potato, chance: 0.3 },
            SingleItemDrop { item_id: ItemId::Potato, chance: 0.1 },
        ],
    }),
)]
pub struct PotatoPlant;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
#[require(
    Id(|| Id(ItemId::PotatoPlantSeed)),
    Name(|| "Potato Plant Seed"),
    ResourceItem,
    GltfData(|| GltfData {
        asset_id: GltfAssetId::PotatoPlant,
        scene_name: Some("stage_0".to_string())
    }),
)]
pub struct PotatoPlantSeed;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
#[require(
    Id(|| Id(ItemId::Potato)),
    Name(|| "Potato"),
    GltfData(|| GltfData {
        asset_id: GltfAssetId::Potato,
        scene_name: None
    }),
    Plant(|| Plant {
        growth_stages: 4,
        growth_requirements: vec![],
        ..Default::default()
    }),
    Health(|| Health::new(1.0))
)]
pub struct Potato;