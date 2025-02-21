use crate::features::ai::PathFollow;
use crate::features::map::map_model::MapData;
use crate::features::path_finding::grid::PathingGridResource;
use crate::features::path_finding::path_finding::{
    spawn_pathfinding_task, PathFollowFinished, PathFollowResult, PathfindingTask,
};
use crate::features::position::WorldPosition;
use beet::prelude::*;
use bevy::math::{IVec2, Vec2};
use bevy::prelude::*;

#[action(go_to_action)]
#[derive(Component, Reflect)]
#[require(ContinueRun, Name(|| "GoToAction"))]
pub struct GoToAction {
    pub(crate) target: IVec2,
}

#[allow(clippy::too_many_arguments)]
fn go_to_action(
    trigger: Trigger<OnRun>,
    world_positions: Query<&WorldPosition>,
    goto_action: Query<&GoToAction>,
    mut commands: Commands,
    map_data: Query<&MapData>,
    pathing_grid: Res<PathingGridResource>,
) {
    let target_agent = trigger.origin;
    let action_entity = trigger.action;
    let world_position = world_positions.get(target_agent).unwrap();
    let goto_action = goto_action.get(action_entity).unwrap();
    let target_coordinate = goto_action.target;
    println!("Ensure path entered NEW, to {}", target_coordinate);
    let target_position = WorldPosition(Vec2::new(
        target_coordinate.x as f32,
        target_coordinate.y as f32,
    ));
    spawn_pathfinding_task(
        &mut commands,
        target_agent,
        &pathing_grid,
        map_data.single(),
        *world_position,
        target_position,
        Some(action_entity),
    );

    // Cleanup on BT remove
    commands.entity(action_entity).observe(
        move |_trigger: Trigger<OnRemove, ContinueRun>, mut commands: Commands| {
            commands
                .entity(target_agent)
                .remove::<PathFollow>()
                .remove::<PathfindingTask>();
        },
    );

    let trigger_clone = trigger.clone();

    commands.entity(target_agent).observe(
        move |path_follow_trigger: Trigger<PathFollowFinished>, mut commands: Commands| {
            if path_follow_trigger.related_task != Some(action_entity) {
                return;
            }

            if commands.get_entity(action_entity).is_none() {
                return;
            }

            match path_follow_trigger.result {
                PathFollowResult::Success => {
                    trigger_clone.trigger_result(&mut commands, RunResult::Success);
                    println!("GoTo action finished, success!");
                }
                PathFollowResult::Failure => {
                    trigger_clone.trigger_result(&mut commands, RunResult::Failure);
                    println!("GoTo action finished, failure!");
                }
            }
        },
    );
}
