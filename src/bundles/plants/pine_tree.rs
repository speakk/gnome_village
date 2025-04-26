use crate::features::path_finding::grid::Solid;
use crate::bundles::{Id, ItemId};
use crate::features::assets::GltfAssetId;
use crate::features::health::Health;
use crate::features::item_drop::ItemDrop;
use crate::features::item_drop::SingleItemDrop;
use crate::features::misc_components::gltf_asset::GltfData;
use crate::features::misc_components::ItemAmount;
use crate::features::plants::Plant;
use bevy::prelude::*;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
#[require(
    Id = Id(ItemId::PineTree),
    Name = Name::new("Pine Tree"),
    GltfData = GltfData {
        asset_id: GltfAssetId::PineTree,
        scene_name: None
    },
    Solid,
    Plant = Plant {
        growth_stages: 4,
        growth_requirements: vec![
            ItemAmount {
                item_id: ItemId::Nitrogen,
                amount: 1
            }
        ],
        ..Default::default()
    },
    Health = Health::new(1.0),
    ItemDrop = ItemDrop {
        item_drops: vec![
            SingleItemDrop { item_id: ItemId::Lumber, chance: 1.0 },
            SingleItemDrop { item_id: ItemId::Lumber, chance: 0.5 },
        ],
    },
)]
pub struct PineTree;
