use crate::bundles::{Id, ItemId, ResourceItem};
use crate::features::assets::GltfAssetId;
use crate::features::misc_components::gltf_asset::GltfData;
use crate::features::plants::Plant;
use bevy::prelude::*;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
#[require(
    Id(|| Id(ItemId::OakTree)),
    Name(|| "Oak Tree"),
    ResourceItem,
    GltfData(|| GltfData {
        asset_id: GltfAssetId::OakTree,
        scene_name: None
    }),
    Plant(|| Plant {
        growth_stages: 4,
        growth_requirements: vec![],
        ..Default::default()
    }),
)]
pub struct OakTree;
