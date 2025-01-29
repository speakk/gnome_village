use crate::bundles::buildables::wooden_wall::WoodenWall;
use crate::bundles::buildables::{BluePrint, BluePrintMaterial, BuildableMaterialHandles};
use crate::features::misc_components::simple_mesh::{SimpleMesh, SimpleMeshHandles};
use crate::features::misc_components::Prototype;
use crate::features::position::WorldPosition;
use bevy::asset::Handle;
use bevy::pbr::{MeshMaterial3d, NotShadowCaster, StandardMaterial};
use bevy::prelude::{
    Added, Changed, Commands, Component, Mesh3d, Query, RemovedComponents, Res, Transform, World,
};
use moonshine_object::{Object, ObjectInstance};
use moonshine_view::{BuildView, ViewCommands, Viewable};

impl BuildView for SimpleMesh {
    fn build(world: &World, object: Object<SimpleMesh>, mut view: ViewCommands<SimpleMesh>) {
        // println!("Building simple mesh VIEW");

        if world.get::<Prototype>(object.entity()).is_some() {
            return;
        }

        let simple_mesh_data = world.get::<SimpleMesh>(object.entity()).unwrap();

        let transform = world.get::<WorldPosition>(object.entity()).unwrap();
        let material_handles = world.get_resource::<BuildableMaterialHandles>().unwrap();
        // TODO: Possibly use own meshes one day, but so far the map cuboid is fine
        let mesh_handles = world.get_resource::<SimpleMeshHandles>().unwrap();
        let mesh_handle = &mesh_handles.0.get(&simple_mesh_data.0).unwrap().clone();

        // TODO: Different materials
        let material_handle = material_handles.wood.clone().unwrap();

        let has_blueprint = world.get::<BluePrint>(object.entity()).is_some();

        let final_material_handle = if has_blueprint {
            //println!("Had blueprint, immediately making blueprint material");
            world
                .get_resource::<BluePrintMaterial>()
                .unwrap()
                .0
                .clone()
                .unwrap()
        } else {
            material_handle.clone()
        };

        view.insert((
            Mesh3d(mesh_handle.clone()),
            MeshMaterial3d(final_material_handle.clone()),
            OriginalMaterial(material_handle.clone()),
            Transform::from_xyz(transform.x, 0.5, transform.y),
        ));

        if has_blueprint {
            view.insert(NotShadowCaster);
        }
    }
}

#[derive(Component, Default)]
pub struct OriginalMaterial(Handle<StandardMaterial>);

pub fn on_add_blueprint(
    query: Query<&Viewable<SimpleMesh>, Added<WorldPosition>>,
    mut materials_query: Query<&mut MeshMaterial3d<StandardMaterial>>,
    blueprint_material: Res<BluePrintMaterial>,
    mut commands: Commands,
) {
    for view in query.iter() {
        println!("blueprint added, making blueprint material");
        let mut material = materials_query.get_mut(view.view().entity()).unwrap();
        material.0 = blueprint_material.clone().unwrap();
        commands
            .entity(view.view().entity())
            .insert(NotShadowCaster);
    }
}

pub fn on_remove_blueprint(
    mut removed: RemovedComponents<BluePrint>,
    mut materials_query: Query<&mut MeshMaterial3d<StandardMaterial>>,
    viewable_query: Query<&Viewable<SimpleMesh>>,
    original_materials_query: Query<&OriginalMaterial>,
    mut commands: Commands,
) {
    removed.read().for_each(|base_entity| {
        if commands.get_entity(base_entity).is_none() {
            return;
        }
        let view = viewable_query.get(base_entity).unwrap();
        let mut material = materials_query.get_mut(view.view().entity()).unwrap();
        let original_material = original_materials_query.get(view.view().entity()).unwrap();
        material.0 = original_material.0.clone();
        commands
            .entity(view.view().entity())
            .remove::<NotShadowCaster>();
    });
}