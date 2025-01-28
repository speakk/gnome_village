use bevy::app::{App, Plugin, Update};
use bevy::prelude::{in_state, IntoSystemConfigs, OnEnter};
use pathfinding::grid::Grid;
use crate::features::path_finding::{grid, path_finding};
use crate::features::path_finding::grid::PathingGridResource;
use crate::features::states::AppState;

pub struct PathFindingPlugin;

impl Plugin for PathFindingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), grid::setup)
            .add_systems(
                Update,
                (
                    grid::update_grid_from_solid_component.run_if(in_state(AppState::InGame)),
                    path_finding::apply_pathfinding_result.run_if(in_state(AppState::InGame)),
                    path_finding::follow_path.run_if(in_state(AppState::InGame)),
                    //test_add_pathfinding_task_to_settler.run_if(in_state(AppState::InGame)),
                ),
            )
            .insert_resource(PathingGridResource(Grid::new(0, 0)));
    }
}