use crate::bundles::{Id, ItemId, ResourceItem};
use crate::features::misc_components::gltf_asset::GltfAsset;
use crate::features::path_finding::Solid;
use crate::features::position::WorldPosition;
use bevy::prelude::*;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
#[require(Id(|| Id(ItemId::Lumber)), WorldPosition, Name(|| "Lumber"), ResourceItem,
    GltfAsset(|| "blender_models/wood.glb"))]
pub struct Lumber;
