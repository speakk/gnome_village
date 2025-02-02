use bevy::prelude::{Added, Changed, Commands, Entity, Or, ParamSet, Query, Trigger, With, Without};
use bevy::hierarchy::Children;
use bevy::utils::HashMap;
use crate::bundles::{Id, Reservations, ResourceItem};
use crate::bundles::settler::Settler;
use crate::features::ai::WorkingOnTask;
use crate::features::misc_components::InWorld;
use crate::features::position::WorldPosition;
use crate::features::tasks::jobs::Job;
use crate::features::tasks::task;
use crate::features::tasks::task::{Status, Task, TaskFinished};

pub fn jobs_changed(tasks_query: Query<Entity, (Or<(Added<Task>, Changed<Task>)>, With<Job>)>) -> bool {
    !tasks_query.is_empty()
}

pub fn assign_jobs(
    mut commands: Commands,
    mut set: ParamSet<(
        Query<(Entity, &Task, Option<&Children>), With<Job>>,
        Query<(Entity, &mut Task, Option<&Children>), With<Job>>,
    )>,
    available_settlers: Query<
        (Entity, &WorldPosition),
        (With<Settler>, Without<WorkingOnTask>, With<InWorld>),
    >,
    mut resources_query: Query<
        (Entity, &WorldPosition, &Id, &mut Reservations),
        (With<ResourceItem>, With<InWorld>),
    >,
    others_query: Query<(Entity, &WorldPosition), (Without<ResourceItem>, Without<Settler>)>,
) {
    let mut ready_tasks: Vec<Entity> = vec![];
    {
        let set0 = set.p0();
        let all_tasks: HashMap<Entity, (Entity, &Task, Option<&Children>)> = set0.iter().map(|x| (x.0, x)).collect();
        
        for (task_entity, task, children) in set0.iter() {
            let ready_task = task::get_available_task(task_entity, task, children, &all_tasks);
            if let Some(ready_task) = ready_task {
                println!("Task {} is ready to go", task_entity);
                ready_tasks.push(ready_task);
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
        let best_agent =
            task.find_best_agent(&mut resources_query, &others_query, &available_settlers);
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
            
            commands.entity(task_entity)
                .observe(move |_trigger: Trigger<TaskFinished>, mut commands: Commands| {
                    println!("Task {} finished, thus removing agent WorkingOnTask", task_entity);
                    commands.entity(best_agent).remove::<WorkingOnTask>();
                });
        }
    }
}

