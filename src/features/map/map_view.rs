use crate::features::map::map_model::{MapData, TileType};
use crate::features::map::water_material::{
    WaterMaterial, NOISE_TEXTURE_1_PATH, NOISE_TEXTURE_2_PATH,
};
use crate::features::misc_components::simple_mesh::{SimpleMeshHandles, SimpleMeshType};
use bevy::asset::{Assets, UntypedHandle};
use bevy::color::Color;
use bevy::math::{UVec2, Vec2};
use bevy::pbr::{MeshMaterial3d, NotShadowCaster, StandardMaterial};
use bevy::prelude::*;
use bevy_platform::collections::HashMap;
use moonshine_object::{Object};
use moonshine_view::{BuildView, ViewCommands};
use noisy_bevy::simplex_noise_2d;

#[derive(Resource, Default, Deref, DerefMut)]
pub struct MapMaterialHandles(pub HashMap<TileType, Vec<UntypedHandle>>);

pub(super) fn create_map_materials(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut map_material_handles: ResMut<MapMaterialHandles>,
    mut water_materials: ResMut<Assets<WaterMaterial>>,
    asset_server: Res<AssetServer>,
    //mut settings: ResMut<WaterSettings>,
) {
    let material_handle1 = materials.add(Color::srgb(0.8, 0.7, 0.6));
    let material_handle2 = materials.add(Color::srgb(0.8, 0.6, 0.5));
    let dirt_material_handles = vec![material_handle1.untyped(), material_handle2.untyped()];

    let water_material_handle = water_materials.add(WaterMaterial {
        //color_1: Color::oklch(75.33, 0.1, 221.29).to_linear(),
        color_1: Color::srgb(0.2, 0.4, 0.6).into(),
        alpha_mode: AlphaMode::Blend,
        noise_texture_1: Some(asset_server.load(NOISE_TEXTURE_1_PATH)),
        noise_texture_2: Some(asset_server.load(NOISE_TEXTURE_2_PATH)),
    });

    map_material_handles.insert(TileType::Dirt, dirt_material_handles);

    map_material_handles.insert(TileType::Water, vec![water_material_handle.untyped()]);
}

impl BuildView for MapData {
    fn build(world: &World, object: Object<Self>, mut view: ViewCommands<Self>) {

        view.insert((Transform::default(), InheritedVisibility::default()));

        let opaque_materials = [TileType::Dirt];
        let translucent_materials = [TileType::Water, TileType::Empty];

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

                        let should_be_cuboid = {
                            if let Some(tile_below) = tile_below {
                                opaque_materials.contains(&tile_type)
                                    && translucent_materials.contains(&tile_below)
                            } else {
                                true
                            }
                        };

                        let mesh_type = if should_be_cuboid {
                            SimpleMeshType::Cuboid
                        } else {
                            SimpleMeshType::Plane
                        };

                        let mesh_handles = world.get_resource::<SimpleMeshHandles>().unwrap();
                        let mesh_handle = mesh_handles.get(&mesh_type).unwrap();
                        let all_material_handles =
                            world.get_resource::<MapMaterialHandles>().unwrap();
                        let material_handles = all_material_handles.get(&tile_type);
                        let Some(material_handles) = material_handles else {
                            continue;
                        };
                        
                        let value = simplex_noise_2d(Vec2::new(x as f32, y as f32) * 0.1);
                        let material_index = (value * material_handles.len() as f32) as usize;
                        let material_handle = material_handles[material_index].clone();

                        let centered_coordinate =
                            map_data.convert_to_centered_coordinate(UVec2::new(x, y));

                        let y = if mesh_type == SimpleMeshType::Cuboid {
                            -0.5
                        } else {
                            0.0
                        };

                        let mut view_entity = view.spawn((
                            Mesh3d(mesh_handle.clone()),
                            NotShadowCaster,
                            Transform::from_xyz(
                                centered_coordinate.x as f32,
                                y,
                                centered_coordinate.y as f32,
                            ),
                        ));

                        match tile_type {
                            TileType::Dirt => {
                                view_entity.insert(MeshMaterial3d(
                                    material_handle.typed::<StandardMaterial>(),
                                ));
                            }
                            TileType::Water => {
                                // TODO: Change this once using a custom material again
                                view_entity.insert(MeshMaterial3d(
                                    material_handle.typed::<WaterMaterial>(),
                                ));
                            }
                            _ => (),
                        }
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
