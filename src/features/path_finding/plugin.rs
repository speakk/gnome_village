use crate::features::path_finding::grid::{react_to_blueprint_removed, PathingGridResource};
use crate::features::path_finding::{grid, path_finding};
use crate::features::states::AppState;
use bevy::app::{App, Plugin, Update};
use bevy::prelude::*;
use pathfinding::grid::Grid;

pub struct PathFindingPlugin;

impl Plugin for PathFindingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), grid::setup)
            .add_systems(
                Update,
                (
                    grid::update_grid_from_solid_component,
                    path_finding::apply_pathfinding_result,
                    path_finding::follow_path,
                    react_to_blueprint_removed,
                    //test_add_pathfinding_task_to_settler.run_if(in_state(AppState::InGame)),
                )
                    .run_if(in_state(AppState::InGame)),
            )
            .insert_resource(PathingGridResource(Grid::new(0, 0)));
    }
}
