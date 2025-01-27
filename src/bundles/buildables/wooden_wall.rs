use bevy::pbr::NotShadowCaster;
use crate::bundles::buildables::{BluePrint, BluePrintMaterial, Buildable, BuildableMaterialHandles, BuildableMeshHandles};
use crate::features::map::map_model::MapData;
use crate::features::misc_components::Prototype;
use crate::features::path_finding::Solid;
use crate::features::position::WorldPosition;
use bevy::prelude::*;
use moonshine_core::prelude::*;
use moonshine_object::Object;
use moonshine_view::{BuildView, RegisterView, ViewCommands, Viewable};
use crate::bundles::{Id, ItemId};

#[derive(Component, Default, Reflect, Clone)]
#[require(Id(|| Id(ItemId::WoodenWall)), WorldPosition, Solid, Name(|| "Wooden Wall"), Buildable)]
#[reflect(Component)]
pub struct WoodenWall;

pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PostUpdate, view_wall_moved)
            .add_systems(PostUpdate, on_add_blueprint)
            .add_systems(PostUpdate, on_remove_blueprint)
            .add_viewable::<WoodenWall>();
    }
}

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

        let has_blueprint = world.get::<BluePrint>(object.entity()).is_some();
        
        let final_material_handle = if has_blueprint {
            println!("Had blueprint, immediately making blueprint material");
            world.get_resource::<BluePrintMaterial>().unwrap().0.clone().unwrap()
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

pub fn on_add_blueprint(query: Query<(&Viewable<WoodenWall>), Added<WorldPosition>>,
                        mut materials_query: Query<&mut MeshMaterial3d<StandardMaterial>>,
                        mut blueprint_material: Res<BluePrintMaterial>,
                        mut commands: Commands
) {
    for view in query.iter() {
        println!("blueprint added, making blueprint material");
        let mut material = materials_query.get_mut(view.view().entity()).unwrap();
        material.0 = blueprint_material.clone().unwrap();
        commands.entity(view.view().entity()).insert(NotShadowCaster);
    }
}

pub fn on_remove_blueprint(mut removed: RemovedComponents<BluePrint>,
                        mut materials_query: Query<&mut MeshMaterial3d<StandardMaterial>>,
                        mut viewable_query: Query<&Viewable<WoodenWall>>,
                           original_materials_query: Query<&OriginalMaterial>,
                           mut commands: Commands
) {
    removed.read().for_each(|base_entity| {
        if commands.get_entity(base_entity).is_none() {
            return;
        }
        let view = viewable_query.get(base_entity).unwrap();
        let mut material = materials_query.get_mut(view.view().entity()).unwrap();
        let original_material = original_materials_query.get(view.view().entity()).unwrap();
        material.0 = original_material.0.clone();
        commands.entity(view.view().entity()).remove::<NotShadowCaster>();
    });
}

pub fn view_wall_moved(
    query: Query<(&WorldPosition, &Viewable<WoodenWall>), Changed<WorldPosition>>,
    mut transform: Query<&mut Transform>,
) {
    for (position, model) in query.iter() {
        let view = model.view();
        let mut transform = transform.get_mut(view.entity()).unwrap();
        *transform = Transform::from_xyz(position.x, 0.5, position.y);
    }
}
