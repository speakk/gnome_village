use crate::features::ai::actions::destruct::DestructAction;
use crate::features::ai::actions::finish_task::FinishTaskAction;
use crate::features::ai::actions::go_to::GoToAction;
use crate::features::ai::{BehaviourTree, WorkingOnTask};
use crate::features::position::WorldPosition;
use crate::features::tasks::task::{Task, TaskCancelled, TaskType};
use beet::prelude::{Fallback, LogOnRun, OnRunAction, Sequence};
use bevy::prelude::*;
use crate::features::ai::actions::fail_task::FailTaskAction;

pub fn create_destruct_tree(
    work_started_query: Query<(&WorkingOnTask, Entity), Added<WorkingOnTask>>,
    world_positions: Query<&WorldPosition>,
    tasks: Query<&Task>,
    mut commands: Commands,
) {
    for (working_on_task, worker_entity) in work_started_query.iter() {
        let task = tasks.get(working_on_task.0).unwrap();
        let task_entity = working_on_task.0;

        if let Some(TaskType::Destruct(destruct_data)) = &task.task_type {
            println!("Had destruct task, creating tree");
            let target_coordinate = world_positions
                .get(destruct_data.target)
                .unwrap()
                .0
                .as_ivec2();

            // TODO: Make mechanism to clean up in case Settler gets despawned
            let tree_entity = commands.spawn((BehaviourTree, Fallback, LogOnRun::new("Destructing fallback"))).id();
            commands.entity(tree_entity).insert(children![
                (
                    LogOnRun::new("Destructing sequence"),
                    Sequence,
                    children![
                        GoToAction {
                            target: target_coordinate,
                        },
                        DestructAction {
                            target: destruct_data.target,
                        },
                        FinishTaskAction {
                            task: working_on_task.0,
                            tree_root: tree_entity
                        }
                    ]
                ),
                FailTaskAction {
                    tree_root: tree_entity,
                    task: working_on_task.0,
                }
            ]).trigger(OnRunAction::new(tree_entity, worker_entity, ()));

            // commands
            //     .entity(tree_entity)
            //     .trigger(OnRunAction::new(tree_entity, worker_entity, ()))
            //     //.observe(move |trigger: Trigger<OnResult>, mut commands: Commands| {
            //     .observe(move |trigger: Trigger<OnResult>, mut commands: Commands| {
            //         if trigger.payload == RunResult::Failure {
            //             //commands.entity(working_on_task.0).despawn();
            //             commands.entity(task_entity).trigger(TaskFailed {
            //                 reason: "Destruct failed".to_string(),
            //             });
            //         }
            //     });

            commands.entity(working_on_task.0).observe(
                move |_trigger: Trigger<TaskCancelled>, mut commands: Commands| {
                    if let Ok(mut entity_commands) = commands.get_entity(tree_entity) {
                        entity_commands.try_despawn();
                    }
                },
            );
        }
    }
}
