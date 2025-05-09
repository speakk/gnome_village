use crate::features::movement::Force;
use crate::features::movement::InverseMass;
use crate::features::position::InterpolatePosition;
use crate::bundles::{Id, ItemId};
use crate::features::assets::GltfAssetId;
use crate::features::inventory::Inventory;
use crate::features::misc_components::gltf_asset::GltfAnimation;
use crate::features::misc_components::gltf_asset::GltfData;
use crate::features::misc_components::preview_carry::PreviewCarry;
use crate::features::movement::Friction;
use crate::features::movement::Velocity;
use bevy::prelude::*;

#[derive(Component, Reflect)]
#[require(
    Id = Id(ItemId::Settler),
    Velocity,
    InverseMass = InverseMass::new(1.0),
    Friction = Friction(0.04),
    Force,
    GltfData = GltfData {
        asset_id: GltfAssetId::Settler,
        scene_name: None
    },
    GltfAnimation = GltfAnimation {
        animation_id: GltfAssetId::Settler,
        animation_indices: vec![0, 1, 2],
        current_animation_index: 0,
        should_play: true
    },
    InterpolatePosition,
    Inventory,
    PreviewCarry,
    Name = Name::new("Settler"))]
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
