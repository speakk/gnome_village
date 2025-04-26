use bevy::asset::Assets;
use bevy::prelude::*;
use bevy_platform::collections::HashMap;

#[derive(Resource, Default, Deref, DerefMut)]
pub struct SimpleMeshHandles(pub HashMap<SimpleMeshType, Handle<Mesh>>);

#[derive(Debug, Reflect, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum SimpleMeshType {
    Plane,
    Cuboid,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
#[derive(Debug)]
pub struct SimpleMesh(pub SimpleMeshType);

pub fn create_simple_meshes(
    mut meshes: ResMut<Assets<Mesh>>,
    mut map_mesh_handles: ResMut<SimpleMeshHandles>,
) {
    let cuboid_handle = meshes.add(Cuboid::default());
    let plane_handle = meshes.add(Plane3d::default().mesh().size(1.0, 1.0));
    map_mesh_handles.insert(SimpleMeshType::Plane, plane_handle);
    map_mesh_handles.insert(SimpleMeshType::Cuboid, cuboid_handle);
}
