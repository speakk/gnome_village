use crate::bundles::buildables::{Buildable, BuildableMaterialHandles, BuildableMeshHandles};
use crate::features::misc_components::{InWorld, Prototype};
use crate::features::path_finding::Solid;
use crate::features::position::WorldPosition;
use bevy::prelude::*;
use moonshine_core::prelude::*;
use moonshine_object::Object;
use moonshine_view::{BuildView, ViewCommands, Viewable};
use crate::features::map::map_model::MapData;

#[derive(Component, Default, Reflect, Clone)]
#[require(WorldPosition, Solid, Name(|| "Wooden Wall"), Buildable)]
#[reflect(Component)]
pub struct WoodenWall;

impl BuildView for WoodenWall {
    fn build(world: &World, object: Object<WoodenWall>, mut view: ViewCommands<WoodenWall>) {
        // if world.get::<InWorld>(object.entity()).is_none() {
        //     return;
        // }

        println!("Building wooden wall VIEW");

        if world.get::<Prototype>(object.entity()).is_some() {
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

pub fn view_wall_moved(query: Query<(&WorldPosition, &Viewable<WoodenWall>), Changed<WorldPosition>>, mut transform: Query<&mut Transform>, map_data: Query<&MapData>) {
    for (position, model) in query.iter() {
        let view = model.view();
        let mut transform = transform.get_mut(view.entity()).unwrap();
        *transform = Transform::from_xyz(position.x, 0.5, position.y);
    }
}