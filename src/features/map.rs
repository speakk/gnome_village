use crate::features::map::map_model::{generate_foliage, generate_map_entity, generate_rocks, generate_test_entities, generate_trees, ReservedCoordinatesHelper};
use crate::features::map::map_view::{create_map_materials, MapMaterialHandles};
use crate::features::states::AppState;
use bevy::app::{App, Plugin};
use bevy::prelude::*;
use moonshine_view::RegisterView;
use crate::features::map::water_material::WaterMaterialPlugin;

pub mod map_model;
pub mod map_view;
mod water_material;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MapMaterialHandles::default())
            .add_viewable::<map_model::MapData>()
            .add_plugins(WaterMaterialPlugin)
            .insert_resource(ReservedCoordinatesHelper::default())
            .add_systems(
                OnEnter(AppState::MapGeneration),
                (
                    create_map_materials,
                    generate_map_entity,
                    generate_rocks,
                    generate_trees,
                    //|world: &mut World| { world.flush() },
                    generate_test_entities,
                    generate_foliage,
                    //generate_reserved_debug,
                    transition_to_in_game,
                )
                    .chain(),
            );
    }
}

fn transition_to_in_game(mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::InGame);
}
