use crate::features::map::map_model::MapData;
use crate::features::path_finding::path_finding::{spawn_pathfinding_task, Path, PathFollowFinished, PathFollowResult};
use crate::features::position::WorldPosition;
use bevy::math::{IVec2, Vec2};
use beet::prelude::*;
use bevy::prelude::*;
use crate::features::ai::{PathFollow, TargetEntity};
use crate::features::path_finding::grid::PathingGridResource;

#[action(escape_from_solid_action)]
#[derive(Component, Reflect)]
#[require(ContinueRun, Name(|| "EscapeFromSolidAction"))]
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
    let target_agent = trigger.origin;
    let world_position = world_positions.get(target_agent).unwrap();
    
    //let trigger_entity = &trigger.entity();
    
    let free_neighbor_coordinate = pathing_grid.get_nearest_available_coordinate(world_position.0.as_ivec2());
    
    if free_neighbor_coordinate.is_none() {
        println!("No free neighbor found, aborting");
        trigger.trigger_result(&mut commands, RunResult::Failure);
        return;
    }
    
    let path = vec![world_position.0.as_ivec2(), free_neighbor_coordinate.unwrap()];
    let path_follow = PathFollow {
        path: Path {
        steps: path,
            related_task: Some(trigger.entity()),
        },
        ..Default::default()
    };

    let trigger_entity = trigger.entity();
    let trigger_clone = trigger.clone(); 

    commands.entity(target_agent).observe(
        move |path_follow_trigger: Trigger<PathFollowFinished>, mut commands: Commands| {
            if path_follow_trigger.related_task != Some(trigger_entity) {
                return;
            }

            match path_follow_trigger.result {
                PathFollowResult::Success => {
                    trigger_clone.trigger_result(&mut commands, RunResult::Success);
                    println!("Escape action finished, success!");
                }
                PathFollowResult::Failure => {
                    trigger_clone.trigger_result(&mut commands, RunResult::Failure);
                    println!("Escape action finished, failure!");
                }
            }
        },
    );
}
