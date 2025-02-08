use beet::prelude::{OnRun, SequenceFlow, TargetEntity};
use bevy::prelude::*;
use crate::features::ai::{BehaviourTree, WorkingOnTask};
use crate::features::ai::actions::build::BuildAction;
use crate::features::ai::actions::finish_task::FinishTaskAction;
use crate::features::ai::actions::go_to::GoToAction;
use crate::features::position::WorldPosition;
use crate::features::tasks::task::{Task, TaskType};

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
            commands
                .spawn((BehaviourTree, SequenceFlow))
                .with_children(|root| {
                    root.spawn((
                        GoToAction {
                            target: target_coordinate,
                        },
                        TargetEntity(worker_entity),
                    ));

                    root.spawn((
                        BuildAction {
                            target: build_data.target,
                        },
                        TargetEntity(worker_entity),
                    ));

                    root.spawn((
                        FinishTaskAction {
                            task: working_on_task.0,
                            tree_root: root.parent_entity(),
                        },
                        TargetEntity(worker_entity),
                    ));
                })
                .trigger(OnRun);
        }
    }
}