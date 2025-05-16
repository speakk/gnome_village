use crate::bundles::settler::Settler;
use crate::bundles::{Id, Reservations, ResourceItem};
use crate::features::ai::WorkingOnTask;
use crate::features::misc_components::InWorld;
use crate::features::position::WorldPosition;
use crate::features::tasks::jobs::Job;
use crate::features::tasks::task;
use crate::features::tasks::task::{
    Status, Task, TaskCancelled, TaskFailed,
    TaskFinished, TaskType,
};
use bevy::prelude::*;
use bevy_platform::collections::HashMap;
use std::time::Duration;
use crate::features::tasks::jobs::build_task::BuildTask;
use crate::features::tasks::jobs::destruct_task::DestructTask;
use crate::features::tasks::sub_tasks::bring_resource_task::BringResourceTask;

// TODO: Restart timers if chunks (to be implemented) nearby change in some way
const JOB_REATTEMPT_DELAY_SECONDS: f32 = 0.2;
const JOB_REATTEMPT_DELAY_MAX: f32 = 5.0;

pub fn jobs_changed(tasks_query: Query<Entity, Or<(Added<Task>, Changed<Task>)>>) -> bool {
    !tasks_query.is_empty()
}

pub fn assign_jobs(
    mut commands: Commands,
    mut set: ParamSet<(
        Query<(Entity, &Task, Option<&Children>)>,
        Query<(Entity, &mut Task, Option<&Children>)>,
    )>,
    jobs: Query<Entity, With<Job>>,
    available_settlers: Query<
        (Entity, &WorldPosition),
        (With<Settler>, Without<WorkingOnTask>, With<InWorld>),
    >,
    mut resources_query: Query<
        (Entity, &WorldPosition, &Id, &mut Reservations),
        (With<ResourceItem>, With<InWorld>),
    >,
    others_query: Query<(Entity, &WorldPosition), (Without<ResourceItem>, Without<Settler>)>,
    mut task_types: Query<(
        Option<&mut BringResourceTask>,
        Option<&mut DestructTask>,
        Option<&mut BuildTask>,
    )>,
) {
    let mut ready_tasks: Vec<Entity> = vec![];
    {
        let set0 = set.p0();
        let all_tasks: HashMap<Entity, (Entity, &Task, Option<&Children>)> =
            set0.iter().map(|x| (x.0, x)).collect();

        for (task_entity, task, children) in set0.iter() {
            if jobs.contains(task_entity) {
                let ready_task = task::get_available_task(task_entity, task, children, &all_tasks);
                if let Some(ready_task) = ready_task {
                    println!("Task {} is ready to go", task_entity);
                    ready_tasks.push(ready_task);
                }
            }
        }
    }

    let mut available_settlers = available_settlers.iter().collect::<Vec<_>>();

    let mut mut_set = set.p1();

    for task_entity in ready_tasks {
        println!("Right going through ready_tasks, task: {:?}", task_entity);
        if available_settlers.is_empty() {
            println!("No settlers available, returning");
            return;
        }

        //let set0 = set.p0();
        let mut task = mut_set.get_mut(task_entity).unwrap().1;
        let (bring_resource_task, destruct_task, build_task) =
            task_types.get_mut(task_entity).unwrap();
        let mut best_agent = None;

        if let Some(mut bring_resource_task) = bring_resource_task {
            best_agent =
                bring_resource_task.score(&mut resources_query, &available_settlers, &others_query);
        } else if let Some(mut destruct_task) = destruct_task {
            best_agent =
                destruct_task.score(&mut resources_query, &available_settlers, &others_query);
        } else if let Some(mut build_task) = build_task {
            best_agent = build_task.score(&mut resources_query, &available_settlers, &others_query);
        }

        if let Some(best_agent) = best_agent {
            println!("Found best agent: {}", best_agent);
            // Delete best_agent from available_settlers
            available_settlers.retain(|&(entity, _)| entity != best_agent);

            task.status = Status::BeingWorkedOn;
            println!(
                "Task {} is being worked on (Inserting WorkingOnTask)",
                task_entity
            );
            commands
                .entity(best_agent)
                .insert(WorkingOnTask(task_entity));

            commands.entity(task_entity).observe(
                move |_trigger: Trigger<TaskFinished>, mut commands: Commands| {
                    println!(
                        "Task {} finished, thus removing agent WorkingOnTask",
                        task_entity
                    );
                    commands.entity(best_agent).remove::<WorkingOnTask>();
                },
            );

            commands.entity(task_entity).observe(
                move |_trigger: Trigger<TaskCancelled>, mut commands: Commands| {
                    println!(
                        "Task {} cancelled, thus removing agent WorkingOnTask",
                        &task_entity.clone()
                    );
                    commands.entity(best_agent).remove::<WorkingOnTask>();
                },
            );

            commands.entity(task_entity).observe(
                move |_trigger: Trigger<TaskFailed>,
                      mut commands: Commands,
                      mut task_data: Query<&mut Task>| {
                    commands.entity(best_agent).remove::<WorkingOnTask>();
                    let mut task_data = task_data.get_mut(task_entity).unwrap();
                    task_data.status = Status::Failed;
                    task_data.cooldown = Some(
                        Duration::from_secs_f32(
                            JOB_REATTEMPT_DELAY_SECONDS * (task_data.failed_tries + 1) as f32,
                        )
                        .min(Duration::from_secs_f32(JOB_REATTEMPT_DELAY_MAX)),
                    );
                    task_data.failed_tries += 1;
                },
            );
        }
    }
}
