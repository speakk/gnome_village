use crate::bundles::buildables::{BluePrint, BluePrintMaterial, BuildableMaterialHandles};
use crate::features::misc_components::simple_mesh::{SimpleMesh, SimpleMeshHandles};
use crate::features::misc_components::Prototype;
use crate::features::position::{InterpolatePosition, WorldPosition};
use bevy::asset::Handle;
use bevy::pbr::{MeshMaterial3d, NotShadowCaster, StandardMaterial};
use bevy::prelude::{Added, Commands, Component, Mesh3d, Query, RemovedComponents, Res, Transform, With, Without, World};
use moonshine_core::kind::Kind;
use moonshine_object::{Object};
use moonshine_view::{BuildView, ViewCommands, Viewable};
use crate::features::inventory::InInventory;
use crate::features::juice::{AddTransformJuice, TransformJuice};

pub struct SimpleMeshValid;

impl Kind for SimpleMeshValid {
    type Filter = (Without<Prototype>, Without<InInventory>, With<WorldPosition>, With<SimpleMesh>);
}

impl BuildView for SimpleMeshValid {
    fn build(world: &World, object: Object<SimpleMeshValid>, mut view: ViewCommands<SimpleMeshValid>) {
        if world.get::<Prototype>(object.entity()).is_some() {
            return;
        }

        let simple_mesh_data = world.get::<SimpleMesh>(object.entity()).unwrap();
        let add_transform_juice = world.get::<AddTransformJuice>(object.entity());

        let transform = world.get::<WorldPosition>(object.entity()).unwrap();
        let material_handles = world.get_resource::<BuildableMaterialHandles>().unwrap();
        let mesh_handles = world.get_resource::<SimpleMeshHandles>().unwrap();
        let mesh_handle = &mesh_handles.0.get(&simple_mesh_data.0).unwrap().clone();

        // TODO: Different materials
        let material_handle = material_handles.wood.clone().unwrap();

        let has_blueprint = world.get::<BluePrint>(object.entity()).is_some();
        let has_interpolate = world.get::<InterpolatePosition>(object.entity()).is_some();

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
        
        if let Some(add_transform_juice) = add_transform_juice {
            view.insert(TransformJuice::from(*add_transform_juice));
        }
        
        if has_interpolate {
            view.insert(InterpolatePosition);
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
        if commands.get_entity(base_entity).is_err() {
            return;
        }
        if let Ok(view) = viewable_query.get(base_entity) {
            let mut material = materials_query.get_mut(view.view().entity()).unwrap();
            let original_material = original_materials_query.get(view.view().entity()).unwrap();
            material.0 = original_material.0.clone();
            commands
                .entity(view.view().entity())
                .remove::<NotShadowCaster>();
        }
    });
}
