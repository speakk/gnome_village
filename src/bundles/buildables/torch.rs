use crate::bundles::buildables::Buildable;
use crate::features::misc_components::GltfAsset;
use crate::features::misc_components::LightSource;
use crate::features::path_finding::Solid;
use crate::features::position::WorldPosition;
use bevy::prelude::*;
use moonshine_core::prelude::*;

#[derive(Component, Default, Reflect)]
#[require(WorldPosition, Solid, Name(|| "Wooden Torch"), Buildable, Save, LightSource, GltfAsset(|| "blender_models/wooden_torch.glb"))]
#[reflect(Component)]
pub struct WoodenTorch;
