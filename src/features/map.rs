// use crate::features::map::map_model::{
//     generate_foliage, generate_map_entity, generate_rocks, generate_test_entities, generate_trees,
//     map_model_plugin,
// };
use crate::features::map::map_view::{create_map_materials, MapMaterialHandles};
use crate::features::map::water_material::WaterMaterialPlugin;
use crate::features::states::AppState;
use bevy::app::{App, Plugin};
use bevy::prelude::*;
use moonshine_view::RegisterView;
use crate::features::map::foliage_instancing::foliage_instancing_plugin;
use crate::features::map::map_model::map_model_plugin;

pub mod map_model;
pub mod map_view;
mod water_material;
mod foliage_instancing;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(MapMaterialHandles::default())
            .add_plugins(WaterMaterialPlugin)
            .add_systems(Startup, create_map_materials)
            .add_plugins(foliage_instancing_plugin)
            .add_plugins(map_model_plugin);
        // app.add_systems(
        //     OnEnter(AppState::MapGeneration),
        //     (
        //         create_map_materials,
        //         generate_map_entity,
        //         generate_rocks,
        //         generate_trees,
        //         generate_test_entities,
        //         generate_foliage,
        //         //generate_reserved_debug,
        //         transition_to_in_game,
        //     )
        //         .chain(),
        // );
    }
}

fn transition_to_in_game(mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::InGame);
}
