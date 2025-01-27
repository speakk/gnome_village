use crate::features::misc_components::simple_mesh::{SimpleMesh, SimpleMeshType};
use crate::bundles::buildables::{BluePrint, BluePrintMaterial, Buildable, BuildableMaterialHandles};
use crate::features::path_finding::Solid;
use crate::features::position::WorldPosition;
use bevy::prelude::*;
use moonshine_core::prelude::*;
use moonshine_object::Object;
use moonshine_view::{BuildView, RegisterView, ViewCommands, Viewable};
use crate::bundles::{Id, ItemId};

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