use crate::bundles::buildables::Buildable;
use crate::bundles::HashMap;
use crate::bundles::{Id, ItemId};
use crate::features::assets::GltfAssetId;
use crate::features::inventory::Inventory;
use crate::features::misc_components::gltf_asset::GltfData;
use crate::features::misc_components::ItemAmount;
use crate::features::path_finding::grid::Solid;
use bevy::prelude::*;

#[derive(Component, Default, Reflect, Clone)]
#[require(
    Id = Id(ItemId::WaterWell),
    Name = Name::new("Water Well"),
    Solid,
    Buildable = Buildable {
        item_requirements: vec![
            ItemAmount {
                item_id: ItemId::Lumber,
                amount: 2,
            }
        ],
        ..Default::default()
    },
    Inventory = Inventory {
        items: HashMap::from([(ItemId::Water, 100)]),
        public_container: true,
    },
    GltfData = GltfData {
        asset_id: GltfAssetId::WaterWell,
        scene_name: None
    }
)]
#[reflect(Component)]
pub struct WaterWell;
