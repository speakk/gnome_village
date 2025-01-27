use crate::bundles::buildables::{Buildable, BuildableMaterialHandles, BuildableMeshHandles};
use crate::features::misc_components::InWorld;
use crate::features::path_finding::Solid;
use crate::features::position::WorldPosition;
use bevy::prelude::*;
use moonshine_core::prelude::*;
use moonshine_object::Object;
use moonshine_view::{BuildView, ViewCommands};

#[derive(Component, Default, Reflect)]
#[require(WorldPosition, Solid, Name(|| "Wooden Wall"), Buildable, Save)]
#[reflect(Component)]
pub struct WoodenWall;

impl BuildView for WoodenWall {
    fn build(world: &World, object: Object<WoodenWall>, mut view: ViewCommands<WoodenWall>) {
        if world.get::<InWorld>(object.entity()).is_none() {
            return;
        }

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
