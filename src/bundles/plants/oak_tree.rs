use crate::features::misc_components::ItemAmount;
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
    Id(|| Id(ItemId::OakTree)),
    Name(|| "Oak Tree"),
    GltfData(|| GltfData {
        asset_id: GltfAssetId::OakTree,
        scene_name: None
    }),
    Plant(|| Plant {
        growth_stages: 4,
        growth_requirements: vec![
            ItemAmount {
                item_id: ItemId::Nitrogen,
                amount: 1
            }
        ],
        ..Default::default()
    }),
    Health(|| Health::new(1.0)),
    ItemDrop(|| ItemDrop {
        item_drops: vec![
            SingleItemDrop { item_id: ItemId::Lumber, chance: 1.0 },
            SingleItemDrop { item_id: ItemId::Lumber, chance: 0.5 },
        ],
    }),
)]
pub struct OakTree;
