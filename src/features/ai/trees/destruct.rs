use crate::features::ai::actions::destruct::DestructAction;
use crate::features::ai::actions::finish_task::FinishTaskAction;
use crate::features::ai::actions::go_to::GoToAction;
use crate::features::ai::{BehaviourTree, WorkingOnTask};
use crate::features::position::WorldPosition;
use crate::features::tasks::task::{Task, TaskCancelled, TaskType};
use beet::prelude::{OnRunAction, Sequence};
use bevy::prelude::*;

pub fn create_destruct_tree(
    work_started_query: Query<(&WorkingOnTask, Entity), Added<WorkingOnTask>>,
    world_positions: Query<&WorldPosition>,
    tasks: Query<&Task>,
    mut commands: Commands,
) {
    for (working_on_task, worker_entity) in work_started_query.iter() {
        let task = tasks.get(working_on_task.0).unwrap();

        if let Some(TaskType::Destruct(destruct_data)) = &task.task_type {
            println!("Had destruct task, creating tree");
            let target_coordinate = world_positions
                .get(destruct_data.target)
                .unwrap()
                .0
                .as_ivec2();

            // TODO: Make mechanism to clean up in case Settler gets despawned
            let tree_entity = commands
                .spawn((BehaviourTree, Sequence))
                .with_children(|root| {
                    root.spawn((GoToAction {
                        target: target_coordinate,
                    },));

                    root.spawn((DestructAction {
                        target: destruct_data.target,
                    },));

                    root.spawn((FinishTaskAction {
                        task: working_on_task.0,
                        tree_root: root.parent_entity(),
                    },));
                })
                .id();

            commands
                .entity(tree_entity)
                .trigger(OnRunAction::new(tree_entity, worker_entity, ()));

            commands.entity(working_on_task.0).observe(
                move |_trigger: Trigger<TaskCancelled>, mut commands: Commands| {
                    if let Some(entity_commands) = commands.get_entity(tree_entity) {
                        entity_commands.try_despawn_recursive();
                    }
                },
            );
        }
    }
}
