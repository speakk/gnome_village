mod wooden_wall;

use crate::features::map::map_view::{MapMaterialHandles, MapMeshHandles, MeshType};
use crate::features::path_finding::Solid;
use crate::features::position::WorldPosition;
use bevy::prelude::*;
use moonshine_core::prelude::*;
use moonshine_view::prelude::*;
use noisy_bevy::simplex_noise_2d;
use crate::bundles::buildables::wooden_wall::{Wall};
use crate::features::map::map_model::TileType;
use crate::features::misc_components::{InWorld, Prototype};
use crate::features::states::AppState;

pub struct BuildablesPlugin;

impl Plugin for BuildablesPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(BuildableMaterialHandles::default())
            .insert_resource(BuildableMeshHandles::default())
            .add_systems(Startup, (setup_buildable_materials, setup_buildable_meshes))
            .add_systems(OnEnter(AppState::InGame), add_buildable_prototypes)
            .add_viewable::<Wall>();
    }
}

#[derive(Resource, Default)]
pub struct BuildableMaterialHandles {
    wood: Option<Handle<StandardMaterial>>
}

#[derive(Resource, Default)]
pub struct BuildableMeshHandles {
    wall: Option<Handle<Mesh>>
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
    map_mesh_handles: Res<MapMeshHandles>
) {
    // Reuse map mesh cuboid for efficiency
    buildable_mesh_handles.wall = map_mesh_handles.get(&MeshType::Cuboid).cloned();
}

pub fn add_buildable_prototypes(mut commands: Commands) {
    println!("Adding buildable prototypes");
    commands.spawn((Wall, Prototype));
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Buildable;

impl BuildView for Wall {
    fn build(world: &World, object: Object<Wall>, mut view: ViewCommands<Wall>) {
        if world.get::<InWorld>(object.entity()).is_none() { return; }
        
        let transform = world.get::<WorldPosition>(object.entity()).unwrap();
        let material_handles = world.get_resource::<BuildableMaterialHandles>().unwrap();
        // TODO: Possibly use own meshes one day, but so far the map cuboid is fine
        let mesh_handles = world.get_resource::<BuildableMeshHandles>().unwrap();
        let mesh_handle = &mesh_handles.wall.clone().unwrap();

        let material_handle = material_handles.wood.clone().unwrap();

        view.insert((
            Mesh3d(mesh_handle.clone()),
            MeshMaterial3d(material_handle),
            Transform::from_xyz(transform.x, 0.5, transform.y),
        ));
    }
}
