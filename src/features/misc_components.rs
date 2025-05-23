pub mod destruct_target;
pub(crate) mod gltf_asset;
pub(crate) mod light_source;
pub mod preview_carry;
pub mod simple_mesh;
mod simple_mesh_view;

use crate::bundles::ItemId;
use crate::features::camera::WorldCamera;
use crate::features::misc_components::destruct_target::{destruct_target_plugin, DestructTarget};
use crate::features::misc_components::gltf_asset::{GltfAssetPlugin, GltfData};
use crate::features::misc_components::preview_carry::{PreviewCarry, PreviewCarryPlugin};
use crate::features::misc_components::simple_mesh::{SimpleMesh, SimpleMeshHandles};
use crate::features::misc_components::simple_mesh_view::{
    on_add_blueprint, on_remove_blueprint, SimpleMeshValid,
};
use crate::features::movement::{MovementIntent};
use crate::features::position::{InterpolatePosition, PreviousWorldPosition, WorldPosition};
use crate::features::states::AppState;
use bevy::app::RunFixedMainLoopSystem::AfterFixedMainLoop;
use bevy::prelude::*;
use bevy_platform::collections::HashMap;
use light_source::LightSource;
use moonshine_core::prelude::Save;
use moonshine_view::{RegisterView, Viewable};

pub struct MiscComponentsPlugin;

impl Plugin for MiscComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SimpleMeshHandles(HashMap::default()))
            .add_systems(Startup, simple_mesh::create_simple_meshes)
            .add_plugins(GltfAssetPlugin)
            .add_plugins(PreviewCarryPlugin)
            .add_plugins(destruct_target_plugin)
            .add_systems(PostUpdate, (on_add_blueprint, on_remove_blueprint))
            .add_systems(
                Update,
                (
                    update_viewable_rotation::<SimpleMesh>,
                    update_viewable_rotation::<GltfData>,
                    update_viewable_rotation::<LightSource>,
                    update_viewable_rotation::<PreviewCarry>,
                    interpolate_rendered_transform::<SimpleMesh>,
                    interpolate_rendered_transform::<GltfData>,
                    interpolate_rendered_transform::<LightSource>,
                    interpolate_rendered_transform::<WorldCamera>,
                    interpolate_rendered_transform::<PreviewCarry>,
                )
                    .run_if(in_state(AppState::InGame)),
            )
            .add_viewable::<SimpleMeshValid>()
            .add_viewable::<LightSource>();
    }
}

fn interpolate_rendered_transform<T>(
    time: Res<Time>,
    query: Query<(
        &WorldPosition,
        &PreviousWorldPosition,
        &Viewable<T>,
        &InterpolatePosition,
    )>,
    mut transforms: Query<&mut Transform>,
) where
    T: Component,
{
    // for (current_world_position, previous_world_position, viewable) in query.iter() {
    //     let previous = previous_world_position.0;
    //     let current = current_world_position.0;
    //     // The overstep fraction is a value between 0 and 1 that tells us how far we are between two fixed timesteps.
    //     let alpha = fixed_time.overstep_fraction();
    //
    //     let rendered_translation = previous.lerp(current, alpha);
    //     let view_entity = viewable.view().entity();
    //     if let Ok(mut transform) = transforms.get_mut(view_entity) {
    //         transform.translation.x = rendered_translation.x;
    //         transform.translation.z = rendered_translation.y;
    //     }
    // }

    for (current_world_position, previous_world_position, viewable, interpolate_position) in
        query.iter()
    {
        let current = current_world_position.0;
        let view_entity = viewable.view().entity();
        if let Ok(mut transform) = transforms.get_mut(view_entity) {
            if let Some(lerp_speed) = interpolate_position.0 {
                let rendered_translation = Vec2::new(transform.translation.x, transform.translation.z)
                    .lerp(current, lerp_speed * time.delta_secs());
                transform.translation.x = rendered_translation.x;
                transform.translation.z = rendered_translation.y;
            } else {
                transform.translation.x = current.x;
                transform.translation.z = current.y;
            }
        }
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

// pub fn viewable_moved<T>(
//     query: Query<(&Transform, &Viewable<T>), (Changed<Transform>, With<InWorld>)>,
//     mut transform: Query<&mut Transform, Without<Viewable<T>>>,
// ) where
//     T: Component,
// {
//     for (position, model) in query.iter() {
//         let view = model.view();
//         let mut transform = transform.get_mut(view.entity()).unwrap();
//         transform.translation = position.translation;
//         //transform.translation = Vec3::new(position.translation.x, 0.0, position.translation.z);
//     }
// }

pub fn update_viewable_rotation<T>(
    query: Query<(&Viewable<T>, &MovementIntent), With<InWorld>>,
    mut transform: Query<&mut Transform>,
    time: Res<Time>,
) where
    T: Component,
{
    for (model, velocity) in query.iter() {
        let view = model.view();
        let mut transform = transform.get_mut(view.entity()).unwrap();
        if velocity.0.length() > 0.05 {
            let mut target_transform = *transform;
            target_transform.look_to(-Vec3::new(velocity.0.x, 0.0, velocity.0.y), Vec3::Y);
            transform
                .rotation
                .smooth_nudge(&target_transform.rotation, 5.0, time.delta_secs());
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct ItemAmount {
    pub item_id: ItemId,
    pub amount: u32,
}
