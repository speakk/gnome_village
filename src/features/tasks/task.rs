use crate::bundles::settler::Settler;
use crate::bundles::{Id, Reservations, ResourceItem};
use crate::features::misc_components::{InWorld, ItemAmount};
use crate::features::position::WorldPosition;
use bevy::ecs::system::SystemState;
use bevy::prelude::Component;
use bevy::prelude::*;
use bevy_platform::collections::HashMap;
use moonshine_core::prelude::ReflectMapEntities;
use moonshine_core::prelude::{MapEntities, Save};
use std::cmp::max;
use std::time::Duration;
use bevy::ecs::query::{QueryData, QueryFilter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum RunType {
    Sequence,
    Parallel,
    Leaf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum Status {
    Ready,
    BeingWorkedOn,
    Finished,
    Failed,
    Cancelled,
}

#[derive(Event)]
pub struct TaskFinished {
    pub task_entity: Entity,
}

#[derive(Event)]
pub struct TaskFailed {
    pub reason: String,
}

pub struct CancelTaskCommand {
    pub task_entity: Entity,
    pub reason: String,
}

pub(super) fn tick_cooldown(mut query: Query<&mut Task>, mut commands: Commands, time: Res<Time>) {
    for mut task in query.iter_mut() {
        if let Some(cooldown) = &mut task.cooldown {
            if *cooldown > Duration::from_secs(0) {
                *cooldown = cooldown
                    .checked_sub(time.delta())
                    .unwrap_or_else(|| Duration::from_secs(0));
            } else {
                task.cooldown = None;
                task.status = Status::Ready;
            }
        }
    }
}

impl Command for CancelTaskCommand {
    fn apply(self, world: &mut World) {
        let task_data = world.get_mut::<Task>(self.task_entity);
        
        let Some(mut task_data) = task_data else {
            return;
        };

        if task_data.status == Status::Finished {
            return;
        }

        task_data.status = Status::Cancelled;

        {
            let mut commands = world.commands();
            commands.entity(self.task_entity).trigger(TaskCancelled {
                reason: self.reason.clone(),
                task_entity: self.task_entity,
            });

            println!(
                "Task: {} cancelled for reason: {}",
                self.task_entity, self.reason
            );
        }
        let mut system_state: SystemState<(Query<&Children>,)> = SystemState::new(world);

        let query = system_state.get(world);
        let child_entities: Vec<Entity> = query.0.iter_descendants(self.task_entity).collect();
        for child in child_entities {
            let mut commands = world.commands();

            commands.entity(child).trigger(TaskCancelled {
                reason: self.reason.clone(),
                task_entity: child,
            });

            println!(
                "Task: {} cancelled for reason: {}",
                self.task_entity, self.reason
            );

            let mut task_data = world.get_mut::<Task>(child).unwrap();
            task_data.status = Status::Cancelled;
        }

        world.flush();
    }
}

#[derive(Event)]
pub struct TaskCancelled {
    pub reason: String,
    pub task_entity: Entity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
#[reflect(MapEntities)]
pub enum DepositTarget {
    Coordinate(IVec2),
    Inventory(Entity),
}

#[derive(Component, Debug, Clone, Reflect)]
#[require(Save)]
#[reflect(Component)]
pub struct Task {
    pub run_type: RunType,
    pub status: Status,
    pub cooldown: Option<Duration>,
    pub failed_tries: u64,
}

impl Default for Task {
    fn default() -> Self {
        Self {
            run_type: RunType::Sequence,
            status: Status::Ready,
            cooldown: None,
            failed_tries: 0,
        }
    }
}

pub fn propagate_finished_upwards(
    mut finished_event_reader: EventReader<TaskFinished>,
    child_of: Query<&ChildOf>,
    children: Query<&Children>,
    mut tasks: Query<&mut Task>,
    mut commands: Commands,
) {
    for finished_event in finished_event_reader.read() {
        println!("Task finished triggered, checking if all children are finished");

        let task_entity = finished_event.task_entity;

        if let Some(parent) = child_of.related(task_entity) {
            let mut all_parent_children = children.relationship_sources(parent);

            let all_children_finished = all_parent_children.all(|child| {
                let task_data = tasks.get(child).unwrap();
                task_data.status == Status::Finished
            });

            if all_children_finished {
                println!("All children finished, triggering parent finished");
                let mut parent_task = tasks.get_mut(parent).unwrap();
                parent_task.status = Status::Finished;
                commands.entity(parent).trigger(TaskFinished {
                    task_entity: parent,
                });
            }
        }
    }
}

pub fn propagate_failed_upwards(
    trigger: Trigger<TaskFailed>,
    child_of: Query<&ChildOf>,
    children: Query<&Children>,
    mut tasks: Query<&mut Task>,
    mut commands: Commands,
) {
    println!("Task failed triggered, checking if all children are finished");

    let task_entity = trigger.target();
    let reason = trigger.reason.clone();

    if let Some(parent) = child_of.related(task_entity) {
        let mut all_parent_children = children.relationship_sources(parent);

        let all_children_failed = all_parent_children.all(|child| {
            if let Ok(child_task) = tasks.get(child) {
                child_task.status == Status::Failed
            } else {
                true
            }
        });

        if all_children_failed {
            println!("All children finished, triggering parent finished");
            if let Ok(mut parent_task) = tasks.get_mut(parent) {
                parent_task.status = Status::Failed;
                commands.entity(parent).trigger(TaskFailed {
                    reason: format!("Child task failed: {}. Reason: {}", task_entity, reason),
                });
            }
        }
    }
}

pub fn get_available_task(
    task_entity: Entity,
    task_data: &Task,
    children: Option<&Children>,
    all_tasks: &HashMap<Entity, (Entity, &Task, Option<&Children>)>,
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
                for child in children.iter() {
                    let (_entity, child_task_data, sub_children) = all_tasks.get(&child).unwrap();

                    if let Some(next_sub_task) =
                        get_available_task(child, child_task_data, *sub_children, all_tasks)
                    {
                        let (_, next_sub_task_data, _) = all_tasks.get(&next_sub_task).unwrap();
                        match next_sub_task_data.status {
                            Status::Ready => return Some(next_sub_task),
                            Status::Finished => continue,
                            _ => return None,
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
                for child in children.iter() {
                    let (_entity, child_task_data, sub_children) = all_tasks.get(&child).unwrap();
                    let next_sub_task =
                        get_available_task(child, child_task_data, *sub_children, all_tasks);
                    if let Some(next_sub_task) = next_sub_task {
                        let (_, next_sub_task_data, _) = all_tasks.get(&next_sub_task).unwrap();
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


#[derive(QueryData)]
#[query_data(mutable, derive(Debug))]
pub struct ResourceQuery {
    pub entity: Entity,
    pub world_position: &'static WorldPosition,
    pub id: &'static Id,
    pub reservations: &'static mut Reservations,
}

#[derive(QueryFilter)]
pub struct ResourceFilter {
    _a: With<ResourceItem>,
    _b: With<InWorld>,
}

pub trait TaskType {
    fn score(
        &mut self,
        resources: &mut Query<
            ResourceQuery,
            ResourceFilter
        >,
        agents: &[(Entity, &WorldPosition)],
        others_query: &Query<(Entity, &WorldPosition), (Without<ResourceItem>, Without<Settler>)>,
    ) -> Option<Entity>;
}
