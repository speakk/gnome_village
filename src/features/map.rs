use crate::features::map::map_model::generate_map_entity;
use crate::features::map::map_view::{
    create_map_meshes_and_materials, MapMaterialHandles, MapMeshHandles,
};
use bevy::app::{App, Plugin, Startup};
use bevy::prelude::IntoSystemConfigs;
use moonshine_view::RegisterView;

pub mod map_model;
mod map_view;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MapMeshHandles::default())
            .insert_resource(MapMaterialHandles::default())
            .add_viewable::<map_model::MapData>()
            .add_systems(
                Startup,
                (create_map_meshes_and_materials, generate_map_entity).chain(),
            );
    }
}
