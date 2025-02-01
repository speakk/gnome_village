use crate::features::map::map_model::MapData;
use crate::features::path_finding::path_finding::{spawn_pathfinding_task, Path, PathFollowFinished, PathFollowResult};
use crate::features::position::WorldPosition;
use beet::prelude::{Action, OnRun, OnRunResult, TargetEntity};
use bevy::math::{IVec2, Vec2};
use bevy::prelude::{Commands, Component, Query, Reflect, Res, Trigger};
use crate::features::ai::PathFollow;
use crate::features::misc_components::gltf_asset::GltfAnimation;
use crate::features::path_finding::grid::PathingGridResource;

#[derive(Component, Action, Reflect)]
#[require(ContinueRun, Name(|| "EscapeFromSolidAction"))]
#[observers(escape_from_solid_action)]
pub struct EscapeFromSolidAction;

#[allow(clippy::too_many_arguments)]
fn escape_from_solid_action(
    trigger: Trigger<OnRun>,
    target_agents: Query<&TargetEntity>,
    world_positions: Query<&WorldPosition>,
    actions: Query<&EscapeFromSolidAction>,
    mut commands: Commands,
    map_data: Query<&MapData>,
    pathing_grid: Res<PathingGridResource>,
) {
    let target_agent = target_agents.get(trigger.entity()).unwrap().0;
    let world_position = world_positions.get(target_agent).unwrap();
    let action = actions.get(trigger.entity()).unwrap();
    let trigger_entity = trigger.entity();
    
    let free_neighbor_coordinate = pathing_grid.get_nearest_available_coordinate(world_position.0.as_ivec2());
    
    if free_neighbor_coordinate.is_none() {
        println!("No free neighbor found, aborting");
        commands
            .entity(trigger_entity)
            .trigger(OnRunResult::failure());
        return;
    }
    
    let path = vec![world_position.0.as_ivec2(), free_neighbor_coordinate.unwrap()];
    let path_follow = PathFollow {
        path: Path {
        steps: path,
            related_task: Some(trigger_entity),
        },
        ..Default::default()
    };
    

    commands.entity(target_agent).observe(
        move |path_follow_trigger: Trigger<PathFollowFinished>, mut commands: Commands| {
            if path_follow_trigger.related_task != Some(trigger_entity) {
                return;
            }

            match path_follow_trigger.result {
                PathFollowResult::Success => {
                    commands
                        .entity(trigger_entity)
                        .trigger(OnRunResult::success());
                    println!("Escape action finished, success!");
                }
                PathFollowResult::Failure => {
                    commands
                        .entity(trigger_entity)
                        .trigger(OnRunResult::failure());
                    println!("Escape action finished, failure!");
                }
            }
        },
    );
}
