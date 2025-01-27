use crate::features::tasks::task::ItemRequirement;
use crate::bundles::buildables::Buildable;
use crate::bundles::{Id, ItemId};
use crate::features::misc_components::gltf_asset::GltfAsset;
use crate::features::misc_components::LightSource;
use crate::features::path_finding::Solid;
use bevy::prelude::*;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
#[require(
    Id(|| Id(ItemId::WoodenTorch)),
    Solid,
    Name(|| "Wooden Torch"),
    Buildable(|| Buildable {
        item_requirements: vec![
            ItemRequirement {
                item_id: ItemId::Lumber,
                amount: 1,
            }
        ]
    }),
    LightSource(|| LightSource { intensity: 50000.0, color: Color::srgb(1.0, 0.9, 0.6) }),
    GltfAsset(|| "blender_models/wooden_torch.glb"))]
pub struct WoodenTorch;
