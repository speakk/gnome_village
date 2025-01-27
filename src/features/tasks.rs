pub mod build_task;
pub mod task;

use crate::features::tasks::build_task::react_to_blueprints;
use crate::features::tasks::task::{RunType, Status, Task};
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

pub fn give_tasks(all_tasks_query: Query<(Entity, &Task, Option<&Children>)>) {
    let mut ready_tasks: Vec<Entity> = vec![];
    for (task_entity, task, children) in all_tasks_query.iter() {
        let ready_task = get_available_task(task_entity, task, children, &all_tasks_query);
        if let Some(ready_task) = ready_task {
            ready_tasks.push(ready_task);
        }
    }

    for task in ready_tasks {
        println!("Task ready: {:?}", task);
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
                    let (_entity, child_task_data, sub_children) =
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
