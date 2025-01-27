use crate::bundles::{Id, ItemId};
use crate::features::inventory::Inventory;
use crate::features::misc_components::gltf_asset::GltfAsset;
use crate::features::movement::Acceleration;
use crate::features::movement::Friction;
use crate::features::movement::Velocity;
use crate::features::position::{PreviousWorldPosition, WorldPosition};
use bevy::prelude::*;

#[derive(Component, Reflect)]
#[require(
    Id(|| Id(ItemId::Settler)),
    WorldPosition,
    PreviousWorldPosition,
    Velocity,
    Friction(|| Friction(0.1)),
    GltfAsset(|| "blender_models/settler.glb"),
    Inventory,
    Name(|| "Settler"))]
#[reflect(Component)]
pub struct Settler {
    carry_capacity: u32,
}

impl Default for Settler {
    fn default() -> Self {
        Self { carry_capacity: 1 }
    }
}

//
// impl BuildView for Settler {
//     fn build(world: &World, object: Object<Settler>, mut view: ViewCommands<Settler>) {
//         println!("Building view for settler");
//         let transform = world.get::<WorldPosition>(object.entity()).unwrap();
//         let asset_server = world.get_resource::<AssetServer>().unwrap();
//         view.insert((
//             SceneRoot(
//                 asset_server
//                     .load(GltfAssetLabel::Scene(0).from_asset("blender_models/settler.glb")),
//             ),
//             Transform::from_xyz(transform.x, 0.0, transform.y),
//             Name::new("Settler view"),
//         ));
//     }
// }
