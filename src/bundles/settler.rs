use crate::features::movement::MovementIntent;
use crate::features::position::InterpolatePosition;
use crate::bundles::{Id, ItemId};
use crate::features::assets::GltfAssetId;
use crate::features::inventory::Inventory;
use crate::features::misc_components::gltf_asset::GltfAnimation;
use crate::features::misc_components::gltf_asset::GltfData;
use crate::features::misc_components::preview_carry::PreviewCarry;
use bevy::prelude::*;

#[derive(Component, Reflect)]
#[require(
    Id = Id(ItemId::Settler),
    MovementIntent,
    GltfData = GltfData {
        asset_id: GltfAssetId::Settler,
        scene_name: None
    },
    GltfAnimation = GltfAnimation {
        animation_id: GltfAssetId::Settler,
        animation_indices: vec![0, 1, 2],
        current_animation_index: 0,
        should_play: true
    },
    InterpolatePosition,
    Inventory,
    PreviewCarry,
    Name = Name::new("Settler"))]
#[reflect(Component)]
pub struct Settler {
    carry_capacity: u32,
}

impl Default for Settler {
    fn default() -> Self {
        Self { carry_capacity: 1 }
    }
}