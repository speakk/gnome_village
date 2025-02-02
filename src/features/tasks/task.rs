use moonshine_core::prelude::ReflectMapEntities;
use crate::bundles::settler::Settler;
use crate::bundles::{Id, ItemId, Reservations, ResourceItem};
use crate::features::ai::trees::bring_resource::score_bring_resource;
use crate::features::misc_components::InWorld;
use crate::features::position::WorldPosition;
use bevy::prelude::Component;
use bevy::prelude::*;
use bevy::utils::HashMap;
use moonshine_core::prelude::{MapEntities, Save};
use crate::features::ai::WorkingOnTask;

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
pub struct TaskFinished(pub TaskFinishedResult);

pub enum TaskFinishedResult {
    Success,
    Failure,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct ItemAmount {
    pub item_id: ItemId,
    pub amount: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum DepositTarget {
    Coordinate(IVec2),
    Inventory(Entity),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct BringResourceRuntimeData {
    pub(crate) concrete_resource_entity: Entity,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Reflect)]
pub struct BringResourceData {
    pub item_requirement: ItemAmount,
    pub target: DepositTarget,
    pub run_time_data: Option<BringResourceRuntimeData>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Reflect)]
pub enum TaskType {
    Build,
    BringResource(BringResourceData),
    GoTo,
}

#[derive(Component, Debug, Clone, Reflect)]
#[require(Save)]
#[reflect(Component, MapEntities)]
pub struct Task {
    pub run_type: RunType,
    pub status: Status,
    pub task_type: Option<TaskType>,
}

// TODO: Wow this seems untenable, perhaps separate Concrete Runtime Data from Task
impl MapEntities for Task {
    fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
        if let Some(TaskType::BringResource(bring_resource_data)) = &mut self.task_type {
            if let Some(run_time_data) = &mut bring_resource_data.run_time_data {
                let entity = &mut run_time_data.concrete_resource_entity;
                *entity = entity_mapper.map_entity(*entity);
            }
        }
    }
}

impl Default for Task {
    fn default() -> Self {
        Self {
            run_type: RunType::Sequence,
            status: Status::Ready,
            task_type: None,
        }
    }
}

impl Task {
    pub fn find_best_agent(
        &mut self,
        resources_query: &mut Query<
            (Entity, &WorldPosition, &Id, &mut Reservations),
            (With<ResourceItem>, With<InWorld>),
        >,
        others_query: &Query<(Entity, &WorldPosition), (Without<ResourceItem>, Without<Settler>)>,
        agents: &Vec<(Entity, &WorldPosition)>,
    ) -> Option<Entity> {
        match &mut self.task_type {
            Some(TaskType::BringResource(bring_resource_data)) => {
                score_bring_resource(resources_query, agents, bring_resource_data, others_query)
            }
            _ => None,
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
                for &child in children.iter() {
                    let (_entity, child_task_data, sub_children) =
                        all_tasks.get(&child).unwrap();
                    let next_sub_task =
                        get_available_task(child, task_data, *sub_children, all_tasks);
                    if let Some(next_sub_task) = next_sub_task {
                        let (_, next_sub_task_data, _) =
                            all_tasks.get(&next_sub_task).unwrap();
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
                        all_tasks.get(&child).unwrap();
                    let next_sub_task =
                        get_available_task(child, task_data, *sub_children, all_tasks);
                    if let Some(next_sub_task) = next_sub_task {
                        let (_, next_sub_task_data, _) =
                            all_tasks.get(&next_sub_task).unwrap();
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