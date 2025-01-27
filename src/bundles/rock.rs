use crate::bundles::{Id, ItemId};
use crate::features::misc_components::simple_mesh::{SimpleMesh, SimpleMeshType};
use crate::features::path_finding::Solid;
use crate::features::position::WorldPosition;
use bevy::prelude::*;

use moonshine_core::prelude::*;
use moonshine_view::prelude::*;
use noisy_bevy::simplex_noise_2d;

pub struct RockPlugin;

impl Plugin for RockPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RockMaterialHandles::default())
            .add_systems(Startup, setup_rock_materials);
    }
}

#[derive(Resource, Default, Deref, DerefMut)]
pub struct RockMaterialHandles(Vec<Handle<StandardMaterial>>);

pub fn setup_rock_materials(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut rock_material_handles: ResMut<RockMaterialHandles>,
) {
    let material_handle1 = materials.add(Color::srgb(0.73, 0.7, 0.7));
    let material_handle2 = materials.add(Color::srgb(0.71, 0.7, 0.71));
    rock_material_handles.0 = vec![material_handle1, material_handle2];
}

#[derive(Component, Default, Reflect)]
#[require(
    Id(|| Id(ItemId::Rock)),
    WorldPosition,
    Solid,
    Save,
    Name(|| "Rock"),
    SimpleMesh(|| SimpleMesh(SimpleMeshType::Cuboid))
)]
#[reflect(Component)]
pub struct Rock;

// impl BuildView for Rock {
//     fn build(world: &World, object: Object<Rock>, mut view: ViewCommands<Rock>) {
//         let transform = world.get::<WorldPosition>(object.entity()).unwrap();
//         let material_handles = world.get_resource::<RockMaterialHandles>().unwrap();
//         // TODO: Possibly use own meshes one day, but so far the map cuboid is fine
//         let mesh_handles = world.get_resource::<MapMeshHandles>().unwrap();
//         let mesh_handle = mesh_handles[&MeshType::Cuboid].clone();
//
//         let value = simplex_noise_2d(Vec2::new(transform.x, transform.y) * 0.12);
//         let material_index = (value * material_handles.len() as f32) as usize;
//         let material_handle = material_handles[material_index].clone();
//
//         view.insert((
//             Mesh3d(mesh_handle.clone()),
//             MeshMaterial3d(material_handle),
//             Transform::from_xyz(transform.x, 0.5, transform.y),
//             Name::new("Rock view"),
//         ));
//     }
// }
