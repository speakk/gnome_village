pub mod build_task;
pub mod task;

use bevior_tree::task::TaskStatus;
use crate::bundles::settler::Settler;
use crate::features::ai::WorkingOnTask;
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

#[derive(Component)]
struct TaskReadyToGo;

pub fn give_tasks(
    mut commands: Commands,
    mut set: ParamSet<(
        Query<(Entity, &Task, Option<&Children>)>,
        Query<(Entity, &mut Task, Option<&Children>)>,
    )>,
    available_settlers: Query<Entity, (With<Settler>, Without<WorkingOnTask>)>,
) {
    let mut ready_tasks: Vec<Entity> = vec![];
    let set0 = set.p0();
    for (task_entity, task, children) in set0.iter() {
        let ready_task = get_available_task(task_entity, &*task, children, &set0);
        if let Some(ready_task) = ready_task {
            println!("Task {} is ready to go", task_entity);
            ready_tasks.push(ready_task);
        }
    }

    let mut available_settlers = available_settlers.iter().collect::<Vec<_>>();

    // TODO: Scoring, for one
    for task_entity in ready_tasks {
        let next_settler = available_settlers.pop();
        println!("Possible next settler: {:?}", next_settler);
        if let Some(next_settler) = next_settler {
            let mut mut_set = set.p1();
            let mut task_data = mut_set.get_mut(task_entity).unwrap().1;
            task_data.status = Status::BeingWorkedOn;
            println!("Task {} is being worked on (Inserting WorkingOnTask)", task_entity);
            commands.entity(next_settler).insert(WorkingOnTask(task_entity));
        } else {
            return;
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
