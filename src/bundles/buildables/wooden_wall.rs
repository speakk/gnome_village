use crate::features::tasks::task::ItemRequirement;
use crate::bundles::buildables::Buildable;
use crate::bundles::{Id, ItemId};
use crate::features::misc_components::simple_mesh::{SimpleMesh, SimpleMeshType};
use crate::features::path_finding::Solid;
use bevy::prelude::*;

#[derive(Component, Default, Reflect, Clone)]
#[require(
    Id(|| Id(ItemId::WoodenWall)),
    Name(|| "Wooden Wall"),
    Solid,
    Buildable(|| Buildable {
        item_requirements: vec![
            ItemRequirement {
                item_id: ItemId::Wood,
                amount: 2,
            }
        ]
    }),
    SimpleMesh(|| SimpleMesh(SimpleMeshType::Cuboid))
)]
#[reflect(Component)]
pub struct WoodenWall;
