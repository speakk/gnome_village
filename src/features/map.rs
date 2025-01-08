use crate::features::map::map_model::{generate_map_entity, generate_test_entities};
use crate::features::map::map_view::{
    create_map_meshes_and_materials, MapMaterialHandles, MapMeshHandles,
};
use bevy::app::{App, Plugin, PostStartup, Startup};
use bevy::prelude::*;
use moonshine_view::RegisterView;
use crate::features::states::AppState;

pub mod map_model;
pub mod map_view;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MapMeshHandles::default())
            .insert_resource(MapMaterialHandles::default())
            .add_viewable::<map_model::MapData>()
            .add_systems(OnEnter(AppState::MapGeneration),
                (create_map_meshes_and_materials, generate_map_entity, generate_test_entities, transition_to_in_game).chain(),
            );
    }
}

fn transition_to_in_game(mut next_state: ResMut<NextState<AppState>>,) {
    next_state.set(AppState::InGame);
}