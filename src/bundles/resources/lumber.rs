use crate::bundles::{Id, ItemId, ResourceItem};
use crate::features::assets::GltfAssetId;
use crate::features::misc_components::gltf_asset::GltfData;
use bevy::prelude::*;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
#[require(Id(|| Id(ItemId::Lumber)),
    Name(|| "Lumber"),
    ResourceItem,
    GltfData(|| GltfData {
        asset_id: GltfAssetId::Lumber,
        scene_name: None
    })
)]
pub struct Lumber;
