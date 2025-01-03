mod features;

use crate::features::camera::CameraPlugin;
use crate::features::movement::MovementPlugin;
use bevy::prelude::*;
use rand::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MapPlugin)
        .add_plugins(features::input::InputPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(MovementPlugin)
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .run();
}

#[derive(Resource)]
pub struct MapSize(IVec2);

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MapSize(IVec2::new(150, 150)))
            .add_systems(Startup, generate_map);
    }
}

fn generate_map(
    map_size: Res<MapSize>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(5.0, 5.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
    ));

    let mut rng = rand::thread_rng();

    let mesh_handle = meshes.add(Cuboid::default());
    let material_handle1 = materials.add(Color::srgb(0.8, 0.7, 0.6));
    let material_handle2 = materials.add(Color::srgb(0.8, 0.6, 0.5));
    let material_handles = [material_handle1, material_handle2];

    for x in -map_size.0.x / 2..map_size.0.x / 2 {
        for y in -map_size.0.y / 2..map_size.0.y / 2 {
            let material_handle = material_handles[rng.gen_range(0..2)].clone();

            // cubes
            commands.spawn((
                Mesh3d(mesh_handle.clone()),
                MeshMaterial3d(material_handle),
                Transform::from_xyz(x as f32, 0.5, y as f32),
            ));
        }
    }

    commands.spawn((PointLight::default(), Transform::from_xyz(3.0, 8.0, 5.0)));
}
