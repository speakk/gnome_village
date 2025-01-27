use crate::features::ai::WorkingOnTask;
use crate::features::tasks::task::{BringResourceData, Task, TaskType};
use bevior_tree::prelude::{delegate_node, ConditionalLoop, Sequence, TaskBridge};
use bevior_tree::BehaviorTreeBundle;
use bevy::prelude::*;

fn create_bring_resource_tree(
    work_started_query: Query<(&WorkingOnTask, Entity), Added<WorkingOnTask>>,
    tasks: Query<&Task>,
    mut commands: Commands,
) {
    for (working_on_task, worker_entity) in work_started_query.iter() {
        let task = tasks.get(working_on_task.0).unwrap();
        if let Some(task_type) = task.task_type {
            if let TaskType::BringResource(bring_resource_data) = task_type {
                commands
                    .entity(worker_entity)
                    .insert(BehaviorTreeBundle::from_root(ConditionalLoop::new(
                        Sequence::new(vec![
                            //Box::new(EnsurePath::new(worker_entity, bring_resource_data.target))
                        ]),
                        |In(_)| true,
                    )));
            }
        }
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
