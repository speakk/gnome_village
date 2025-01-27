use bevy::math::{IVec2, Vec2};
use bevy::prelude::{Commands, Entity, In, Query, Res, With};
use bevior_tree::prelude::{delegate_node, TaskBridge, TaskEvent, TaskStatus};
use bevior_tree::node::NodeResult;
use crate::bundles::settler::Settler;
use crate::features::ai::PathFollow;
use crate::features::map::map_model::MapData;
use crate::features::path_finding::{spawn_pathfinding_task, PathfindingTask, PathingGridResource};
use crate::features::position::WorldPosition;

#[delegate_node(delegate)]
pub struct GoTo {
    delegate: TaskBridge,
}

const TARGET_DISTANCE_THRESHOLD: f32 = 1.5;

impl GoTo {
    pub fn new(target_coordinate: IVec2) -> Self {
        let checker = move |entity: In<Entity>, query: Query<(&WorldPosition, Option<&PathfindingTask>, Option<&PathFollow>), With<Settler>>| {
            let (world_position, path_finding_task, path_follow) = query.get(entity.0).unwrap();
            
            if path_finding_task.is_some() || path_follow.is_some() {
                let distance_to_target = world_position.0.distance(target_coordinate.as_vec2());

                return if distance_to_target > TARGET_DISTANCE_THRESHOLD {
                    TaskStatus::Running
                } else {
                    println!("Reached target, returning success");
                    TaskStatus::Complete(NodeResult::Success)
                }
            }

            println!("Didn't have path, returning failure");
            TaskStatus::Complete(NodeResult::Failure)
        };

        let task = TaskBridge::new(checker)
            .on_event(TaskEvent::Enter, move |entity: In<Entity>,
                                              mut commands: Commands,
                                              map_data: Query<&MapData>,
                                              pathing_grid: Res<PathingGridResource>,
                                              world_position: Query<&WorldPosition>| {
                println!("Ensure path entered, to {}", target_coordinate);
                let target_position = WorldPosition(Vec2::new(target_coordinate.x as f32, target_coordinate.y as f32));
                spawn_pathfinding_task(&mut commands, *entity, &pathing_grid, map_data.single(), *world_position.get(*entity).unwrap(), target_position);
            });

        Self { delegate: task }
    }
}