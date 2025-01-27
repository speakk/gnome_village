pub(crate) mod gltf_asset;
pub mod simple_mesh;
mod simple_mesh_view;

use crate::features::misc_components::gltf_asset::GltfAssetPlugin;
use crate::features::misc_components::simple_mesh::{SimpleMesh, SimpleMeshHandles};
use crate::features::misc_components::simple_mesh_view::{
    on_add_blueprint, on_remove_blueprint, view_wall_moved,
};
use crate::features::position::WorldPosition;
use bevy::prelude::*;
use bevy::utils::HashMap;
use moonshine_object::{Object, ObjectInstance};
use moonshine_view::{BuildView, RegisterView, ViewCommands};

pub struct MiscComponentsPlugin;

impl Plugin for MiscComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SimpleMeshHandles(HashMap::default()))
            .add_systems(Startup, simple_mesh::create_simple_meshes)
            .add_plugins(GltfAssetPlugin)
            .add_systems(
                PostUpdate,
                (on_add_blueprint, on_remove_blueprint, view_wall_moved),
            )
            .add_viewable::<SimpleMesh>()
            .add_viewable::<LightSource>();
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct InWorld;

// Explicitly not-in-game, for use in menus etc
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Prototype;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct LightSource {
    pub intensity: f32,
    pub color: Color,
}

impl Default for LightSource {
    fn default() -> Self {
        Self {
            intensity: 100000.0,
            color: Color::WHITE,
        }
    }
}

impl BuildView for LightSource {
    fn build(world: &World, object: Object<LightSource>, mut view: ViewCommands<LightSource>) {
        if world.get::<Prototype>(object.entity()).is_some() {
            return;
        }
        println!("Building light source");

        let transform = world.get::<WorldPosition>(object.entity()).unwrap();
        let light_source = world.get::<LightSource>(object.entity()).unwrap();

        view.insert((
            PointLight {
                color: light_source.color,
                intensity: light_source.intensity,
                range: 2.0,
                shadows_enabled: true,
                ..default()
            },
            Transform::from_xyz(transform.x, 1.5, transform.y),
        ));
    }
}
