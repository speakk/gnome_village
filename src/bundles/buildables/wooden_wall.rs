use crate::bundles::buildables::Buildable;
use crate::bundles::{Id, ItemId};
use crate::features::misc_components::simple_mesh::{SimpleMesh, SimpleMeshType};
use crate::features::path_finding::Solid;
use crate::features::position::WorldPosition;
use bevy::prelude::*;

#[derive(Component, Default, Reflect, Clone)]
#[require(
    Id(|| Id(ItemId::WoodenWall)),
    WorldPosition,
    Solid,
    Name(|| "Wooden Wall"),
    Buildable,
    SimpleMesh(|| SimpleMesh(SimpleMeshType::Cuboid))
)]
#[reflect(Component)]
pub struct WoodenWall;
