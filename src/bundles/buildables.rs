pub mod torch;
pub mod wooden_wall;

use crate::bundles::buildables::torch::WoodenTorch;
use crate::bundles::buildables::wooden_wall::WoodenWall;
use crate::features::map::map_view::{MapMeshHandles, MeshType};
use crate::features::misc_components::Prototype;
use crate::features::states::AppState;
use bevy::prelude::*;
use moonshine_core::prelude::Save;
use moonshine_view::prelude::*;

pub struct BuildablesPlugin;

impl Plugin for BuildablesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BuildableMaterialHandles::default())
            .insert_resource(BuildableMeshHandles::default())
            .add_systems(Startup, (setup_buildable_materials, setup_buildable_meshes))
            .add_systems(OnEnter(AppState::InGame), add_buildable_prototypes)
            .add_viewable::<WoodenWall>();
    }
}

#[derive(Resource, Default)]
pub struct BuildableMaterialHandles {
    wood: Option<Handle<StandardMaterial>>,
}

#[derive(Resource, Default)]
pub struct BuildableMeshHandles {
    wall: Option<Handle<Mesh>>,
}

pub fn setup_buildable_materials(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut buildable_material_handles: ResMut<BuildableMaterialHandles>,
) {
    let wood_material = materials.add(Color::srgb(0.6, 0.4, 0.37));
    buildable_material_handles.wood = Some(wood_material);
}

pub fn setup_buildable_meshes(
    mut buildable_mesh_handles: ResMut<BuildableMeshHandles>,
    map_mesh_handles: Res<MapMeshHandles>,
) {
    // Reuse map mesh cuboid for efficiency
    buildable_mesh_handles.wall = map_mesh_handles.get(&MeshType::Cuboid).cloned();
}

pub fn add_buildable_prototypes(mut commands: Commands) {
    println!("Adding buildable prototypes");
    commands.spawn((WoodenWall, Prototype)).remove::<Save>();
    commands.spawn((WoodenTorch, Prototype)).remove::<Save>();
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Buildable;
