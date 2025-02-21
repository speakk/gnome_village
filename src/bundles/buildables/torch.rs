use crate::bundles::buildables::Buildable;
use crate::bundles::{Id, ItemId};
use crate::features::assets::GltfAssetId;
use crate::features::misc_components::gltf_asset::GltfData;
use crate::features::misc_components::light_source::LightSource;
use crate::features::misc_components::ItemAmount;
use crate::features::path_finding::grid::Solid;
use bevy::prelude::*;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
#[require(
    Id(|| Id(ItemId::WoodenTorch)),
    Solid,
    Name(|| "Wooden Torch"),
    Buildable(|| Buildable {
        item_requirements: vec![
            ItemAmount {
                item_id: ItemId::Lumber,
                amount: 1,
            }
        ],
        ..Default::default()
    }),
    LightSource(|| LightSource { intensity: 50000.0, color: Color::srgb(1.0, 0.9, 0.6) }),
    GltfData(|| GltfData {
        asset_id: GltfAssetId::WoodenTorch,
        scene_name: None
    }))]
pub struct WoodenTorch;
