use crate::bundles::{Id, ItemId};
use crate::features::misc_components::gltf_asset::GltfAsset;
use crate::features::path_finding::Solid;
use crate::features::position::WorldPosition;
use bevy::prelude::*;
use moonshine_core::prelude::*;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
#[require(Id(|| Id(ItemId::Lumber)), WorldPosition, Solid, Name(|| "Lumber"),
    GltfAsset(|| "blender_models/wood.glb"))]
pub struct Lumber;
