use crate::features::ai::{PathFollow, WorkingOnTask};
use crate::features::tasks::task::{BringResourceData, DepositTarget, Task, TaskType};
use bevior_tree::prelude::{delegate_node, ConditionalLoop, Sequence, TaskBridge};
use bevior_tree::BehaviorTreeBundle;
use bevior_tree::node::NodeResult;
use bevior_tree::task::{TaskEvent, TaskStatus};
use bevy::prelude::*;
use crate::bundles::settler::Settler;
use crate::features::map::map_model::MapData;
use crate::features::path_finding::{spawn_pathfinding_task, PathfindingTask, PathingGridResource};
use crate::features::position::WorldPosition;

pub fn create_bring_resource_tree(
    work_started_query: Query<(&WorkingOnTask, Entity), Added<WorkingOnTask>>,
    tasks: Query<&Task>,
    mut commands: Commands,
) {
    for (working_on_task, worker_entity) in work_started_query.iter() {
        let task = tasks.get(working_on_task.0).unwrap();
        println!("Found WorkingOnTask");
        
        if let Some(task_type) = task.task_type {
            println!("Task type: {:?}", task_type);
            if let TaskType::BringResource(bring_resource_data) = task_type {
                println!("Had BringResource task, creating tree");
                let target_coordinate = match bring_resource_data.target {
                    DepositTarget::Coordinate(coordinate) => coordinate,
                    DepositTarget::Inventory(inventory_entity) => panic!(
                        "Inventory target is not supported yet. Inventory entity: {}",
                        inventory_entity
                    )
                };
                
                commands
                    .entity(worker_entity)
                    .insert(BehaviorTreeBundle::from_root(
                        Sequence::new(vec![
                            Box::new(EnsurePath::new(target_coordinate))
                        ]),
                    ));
            }
        }
    }
}

#[delegate_node(delegate)]
struct EnsurePath {
    delegate: TaskBridge,
}

impl EnsurePath {
    pub fn new(target_coordinate: IVec2) -> Self {
        let checker = move |entity: In<Entity>, query: Query<(Option<&PathfindingTask>, Option<&PathFollow>), With<Settler>>| {
            let (path_finding_task, path_follow) = query.get(entity.0).unwrap();
            if path_finding_task.is_some() || path_follow.is_some() {
                println!("Already have path or path task, skipping");
                return TaskStatus::Complete(NodeResult::Success);
            }

            println!("Didn't have path, returning failure");
            return TaskStatus::Complete(NodeResult::Failure);
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

// use bevior_tree::prelude::{delegate_node, TaskBridge};
// use bevy::prelude::*;
// use crate::features::tasks::task::{BringResourceData, TaskType};
//
// // Task to wait until player get near.
// // Task trait is available for making your task, delegating core methods to TaskImpl.
// #[delegate_node(delegate)]
// struct BringResourceTree {
//     delegate: TaskBridge,
// }
//
// impl BringResourceTree {
//     pub fn new(bring_resource_data: BringResourceData) -> Self {
//         let checker = move |In(entity): In<Entity>, param: Query<&Transform>| {
//             let distance = get_distance(entity, target, param);
//             // Return `Failure` if target is out of range.
//             match distance <= range {
//                 true => TaskStatus::Running,
//                 false => TaskStatus::Complete(NodeResult::Failure),
//             }
//         };
//     }
// }
