use crate::features::ai::actions::build::BuildAction;
use crate::features::ai::actions::finish_task::FinishTaskAction;
use crate::features::ai::actions::go_to::GoToAction;
use crate::features::ai::{BehaviourTree, WorkingOnTask};
use crate::features::position::WorldPosition;
use crate::features::tasks::task::{Task, TaskCancelled, TaskType};
use beet::prelude::{OnRunAction, Sequence};
use bevy::prelude::*;

pub fn create_build_tree(
    work_started_query: Query<(&WorkingOnTask, Entity), Added<WorkingOnTask>>,
    world_positions: Query<&WorldPosition>,
    tasks: Query<&Task>,
    mut commands: Commands,
) {
    for (working_on_task, worker_entity) in work_started_query.iter() {
        let task = tasks.get(working_on_task.0).unwrap();
        println!("Found WorkingOnTask");

        if let Some(TaskType::Build(build_data)) = &task.task_type {
            println!("Had Build task, creating BUILD tree");
            let target_coordinate = world_positions.get(build_data.target).unwrap().0.as_ivec2();

            // TODO: Make mechanism to clean up in case Settler gets despawned
            let tree_entity = commands
                .spawn((BehaviourTree, Sequence))
                .with_children(|root| {
                    root.spawn((GoToAction {
                        target: target_coordinate,
                    },));

                    root.spawn((BuildAction {
                        target: build_data.target,
                    },));

                    root.spawn((FinishTaskAction {
                        task: working_on_task.0,
                        tree_root: root.target_entity(),
                    },));
                })
                .id();

            commands
                .entity(tree_entity)
                .trigger(OnRunAction::new(tree_entity, worker_entity, ()));

            commands.entity(working_on_task.0).observe(
                move |_trigger: Trigger<TaskCancelled>, mut commands: Commands| {
                    commands.entity(tree_entity).try_despawn();
                },
            );
        }
    }
}
