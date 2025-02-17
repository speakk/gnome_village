pub(crate) mod gltf_asset;
pub mod simple_mesh;
mod simple_mesh_view;
pub(crate) mod light_source;

use crate::features::misc_components::gltf_asset::{GltfData, GltfAssetPlugin};
use crate::features::misc_components::simple_mesh::{SimpleMesh, SimpleMeshHandles};
use crate::features::misc_components::simple_mesh_view::{on_add_blueprint, on_remove_blueprint};
use crate::features::position::{WorldPosition};
use bevy::prelude::*;
use bevy::utils::HashMap;
use moonshine_core::prelude::Save;
use moonshine_object::{Object, ObjectInstance};
use moonshine_view::{BuildView, RegisterView, ViewCommands, Viewable};
use light_source::LightSource;
use crate::features::movement::Velocity;

pub struct MiscComponentsPlugin;

impl Plugin for MiscComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SimpleMeshHandles(HashMap::default()))
            .add_systems(Startup, simple_mesh::create_simple_meshes)
            .add_plugins(GltfAssetPlugin)
            .add_systems(PostUpdate, (on_add_blueprint, on_remove_blueprint))
            .add_systems(
                PostUpdate,
                (
                    viewable_moved::<SimpleMesh>,
                    update_viewable_rotation::<SimpleMesh>,
                    viewable_moved::<GltfData>,
                    update_viewable_rotation::<GltfData>,
                    viewable_moved::<LightSource>,
                    update_viewable_rotation::<LightSource>,
                ),

            )
            .add_viewable::<SimpleMesh>()
            .add_viewable::<LightSource>();
    }
}

#[derive(Component, Default, Reflect)]
#[require(Save)]
#[reflect(Component)]
pub struct InWorld;

// Explicitly not-in-game, for use in menus etc
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Prototype;

pub fn viewable_moved<T>(
    query: Query<(&WorldPosition, &Viewable<T>), Changed<WorldPosition>>,
    mut transform: Query<&mut Transform>,
) where
    T: Component,
{
    for (position, model) in query.iter() {
        let view = model.view();
        let mut transform = transform.get_mut(view.entity()).unwrap();
        transform.translation = Vec3::new(position.x, 0.0, position.y);
    }
}

pub fn update_viewable_rotation<T>(
    query: Query<(&Viewable<T>, &Velocity)>,
    mut transform: Query<&mut Transform>,
    time: Res<Time>
) where T: Component {
    for (model, velocity) in query.iter() {
        let view = model.view();
        let mut transform = transform.get_mut(view.entity()).unwrap();
        if velocity.0.length() > 0.5 {
            let mut target_transform = *transform;
            target_transform.look_to(-Vec3::new(velocity.0.x, 0.0, velocity.0.y), Vec3::Y);
            transform.rotation.smooth_nudge(&target_transform.rotation, 5.0, time.delta_secs());
        }
    }
}