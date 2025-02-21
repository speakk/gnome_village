use crate::bundles::{Id, ItemId, ResourceItem};
use crate::features::misc_components::gltf_asset::GltfData;
use crate::features::assets::GltfAssetId;
use bevy::prelude::*;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
#[require(Id(|| Id(ItemId::OakTree)), Name(|| "Oak Tree"), ResourceItem,
    GltfData(|| GltfData {
        asset_id: GltfAssetId::OakTree,
        scene_name: None
    }))]
pub struct OakTree;
