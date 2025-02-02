pub mod jobs;
pub mod task;

use crate::bundles::settler::Settler;
use crate::bundles::{Id, Reservations, ResourceItem};
use crate::features::ai::WorkingOnTask;
use crate::features::misc_components::InWorld;
use crate::features::position::WorldPosition;
use crate::features::tasks::jobs::build_task::react_to_blueprints;
use crate::features::tasks::task::{RunType, Status, Task, TaskFinished};
use bevy::prelude::*;

pub struct TasksPlugin;

impl Plugin for TasksPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, react_to_blueprints)
            .add_systems(Update, give_tasks.run_if(tasks_changed));
    }
}

fn tasks_changed(tasks_query: Query<Entity, Or<(Added<Task>, Changed<Task>)>>) -> bool {
    !tasks_query.is_empty()
}

pub fn give_tasks(
    mut commands: Commands,
    mut set: ParamSet<(
        Query<(Entity, &Task, Option<&Children>)>,
        Query<(Entity, &mut Task, Option<&Children>)>,
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
        for (task_entity, task, children) in set0.iter() {
            let ready_task = get_available_task(task_entity, task, children, &set0);
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

fn get_available_task(
    task_entity: Entity,
    task_data: &Task,
    children: Option<&Children>,
    all_tasks_query: &Query<(Entity, &Task, Option<&Children>)>,
) -> Option<Entity> {
    match task_data.run_type {
        RunType::Leaf => {
            if task_data.status == Status::Ready {
                Some(task_entity)
            } else {
                None
            }
        }
        RunType::Sequence => {
            if let Some(children) = children {
                for &child in children.iter() {
                    let (_entity, child_task_data, sub_children) =
                        all_tasks_query.get(child).unwrap();
                    let next_sub_task =
                        get_available_task(child, task_data, sub_children, all_tasks_query);
                    if let Some(next_sub_task) = next_sub_task {
                        let (_, next_sub_task_data, _) =
                            all_tasks_query.get(next_sub_task).unwrap();
                        return if next_sub_task_data.status == Status::BeingWorkedOn {
                            None
                        } else {
                            Some(next_sub_task)
                        };
                    } else if child_task_data.status == Status::Finished {
                        continue;
                    }

                    return None;
                }
            }

            None
        }
        RunType::Parallel => {
            if let Some(children) = children {
                for &child in children.iter() {
                    let (_entity, _child_task_data, sub_children) =
                        all_tasks_query.get(child).unwrap();
                    let next_sub_task =
                        get_available_task(child, task_data, sub_children, all_tasks_query);
                    if let Some(next_sub_task) = next_sub_task {
                        let (_, next_sub_task_data, _) =
                            all_tasks_query.get(next_sub_task).unwrap();
                        if next_sub_task_data.status == Status::Ready {
                            return Some(next_sub_task);
                        }
                    }
                }
            }

            None
        }
    }
}
