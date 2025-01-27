use crate::features::map::map_model::{MapData, TileType};
use bevy::asset::{Assets, Handle};
use bevy::color::Color;
use bevy::hierarchy::{BuildChildren, ChildBuild};
use bevy::math::{UVec2, Vec2};
use bevy::pbr::{MeshMaterial3d, StandardMaterial};
use bevy::prelude::{
    Cuboid, Deref, DerefMut, InheritedVisibility, Mesh, Mesh3d, Meshable, Plane3d, ResMut,
    Resource, Transform, World,
};
use bevy::utils::HashMap;
use moonshine_object::{Object, ObjectInstance};
use moonshine_view::prelude::*;
use moonshine_view::{BuildView, ViewCommands};
use noisy_bevy::simplex_noise_2d;

#[derive(Resource, Default, Deref, DerefMut)]
pub struct MapMeshHandles(pub HashMap<MeshType, Handle<Mesh>>);

#[derive(Resource, Default, Deref, DerefMut)]
pub struct MapMaterialHandles(pub HashMap<TileType, Vec<Handle<StandardMaterial>>>);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum MeshType {
    Plane,
    Cuboid,
}

pub fn create_map_meshes_and_materials(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut map_mesh_handles: ResMut<MapMeshHandles>,
    mut map_material_handles: ResMut<MapMaterialHandles>,
) {
    let cuboid_handle = meshes.add(Cuboid::default());
    let plane_handle = meshes.add(Plane3d::default().mesh().size(1.0, 1.0));
    map_mesh_handles.insert(MeshType::Plane, plane_handle);
    map_mesh_handles.insert(MeshType::Cuboid, cuboid_handle);

    let material_handle1 = materials.add(Color::srgb(0.8, 0.7, 0.6));
    let material_handle2 = materials.add(Color::srgb(0.8, 0.6, 0.5));
    let dirt_material_handles = vec![material_handle1, material_handle2];
    map_material_handles.insert(TileType::Dirt, dirt_material_handles);
}

impl BuildView for MapData {
    fn build(world: &World, object: Object<Self>, mut view: ViewCommands<Self>) {
        println!("Building map view for object: {:?}", object);

        view.insert((Transform::default(), InheritedVisibility::default()));

        view.with_children(|view| {
            if let Some(map_data) = world.get::<MapData>(object.entity()) {
                //println!("Map data: {:?}", map_data.data);
                for x in 0..map_data.size.x {
                    for y in 0..map_data.size.y {
                        let tile_type_option =
                            map_data.get_tile_type_non_centered(UVec2::new(x, y));

                        let Some(tile_type) = tile_type_option else {
                            continue;
                        };

                        let tile_below = map_data.get_tile_type_non_centered(UVec2::new(x, y + 1));
                        let has_tile_below =
                            tile_below.is_some() && tile_below.unwrap() != TileType::Empty;

                        let mesh_type = if has_tile_below {
                            MeshType::Cuboid
                        } else {
                            MeshType::Plane
                        };

                        let mesh_handles = world.get_resource::<MapMeshHandles>().unwrap();
                        let mesh_handle = mesh_handles.get(&mesh_type).unwrap();
                        let all_material_handles =
                            world.get_resource::<MapMaterialHandles>().unwrap();
                        let material_handles = all_material_handles[&tile_type].clone();
                        let value = simplex_noise_2d(Vec2::new(x as f32, y as f32) * 0.1);
                        let material_index = (value * material_handles.len() as f32) as usize;
                        let material_handle = material_handles[material_index].clone();

                        let centered_coordinate =
                            map_data.convert_to_centered_coordinate(UVec2::new(x, y));
                        //
                        // println!(
                        //     "Inserting! Centered coordinate: {:?}, handles: {:?}, {:?}",
                        //     centered_coordinate, mesh_handle, material_handle
                        // );
                        view.spawn((
                            Mesh3d(mesh_handle.clone()),
                            MeshMaterial3d(material_handle),
                            Transform::from_xyz(
                                centered_coordinate.x as f32,
                                0.5,
                                centered_coordinate.y as f32,
                            ),
                        ));
                    }
                }
            }
        });
    }
}

// fn generate_map(
//     map_size: Res<MapSize>,
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     let mut rng = rand::thread_rng();
//
//     for x in -map_size.0.x / 2..map_size.0.x / 2 {
//         for y in -map_size.0.y / 2..map_size.0.y / 2 {
//
//             // cubes
//             commands.spawn((
//                 Mesh3d(mesh_handle.clone()),
//                 MeshMaterial3d(material_handle),
//                 Transform::from_xyz(x as f32, 0.5, y as f32),
//             ));
//         }
//     }
//
//     commands.spawn((PointLight::default(), Transform::from_xyz(3.0, 8.0, 5.0)));
// }
