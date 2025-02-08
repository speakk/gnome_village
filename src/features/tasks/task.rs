use crate::bundles::settler::Settler;
use crate::bundles::{Id, ItemId, Reservations, ResourceItem};
use crate::features::ai::trees::bring_resource::score_bring_resource;
use crate::features::ai::WorkingOnTask;
use crate::features::misc_components::InWorld;
use crate::features::position::WorldPosition;
use crate::features::tasks::jobs::build_task::score_build;
use bevy::hierarchy::HierarchyEvent;
use bevy::prelude::Component;
use bevy::prelude::*;
use bevy::utils::HashMap;
use moonshine_core::prelude::ReflectMapEntities;
use moonshine_core::prelude::{MapEntities, Save};

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
    pub result: TaskFinishedResult,
    pub task_entity: Entity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
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
pub struct BuildData {
    pub target: Entity,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Reflect)]
pub enum TaskType {
    Build(BuildData),
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

// pub fn handle_task_added(query: Query<(Entity, &Task), Added<Task>>, mut commands: Commands) {
//     for (entity, task) in query.iter() {
//         commands.entity(entity).observe(propagate_finished_upwards);
//     }
// }

pub fn propagate_finished_upwards(
    mut finished_event_reader: EventReader<TaskFinished>,
    parents: Query<&Parent>,
    children: Query<&Children>,
    mut tasks: Query<&mut Task>,
    mut commands: Commands,
) {
    for finished_event in finished_event_reader.read() {
        println!("Task finished triggered, checking if all children are finished");
        if finished_event.result != TaskFinishedResult::Success {
            continue;
        }
        
        let task_entity = finished_event.task_entity;

        if let Some(parent) = parents.parent(task_entity) {
            let all_parent_children = children.children(parent);

            let all_children_finished = all_parent_children.iter().all(|child| {
                let task_data = tasks.get(*child).unwrap();
                task_data.status == Status::Finished
            });

            if all_children_finished {
                println!("All children finished, triggering parent finished");
                let mut parent_task = tasks.get_mut(parent).unwrap();
                parent_task.status = Status::Finished;
                commands.entity(parent).trigger(TaskFinished {
                    result: TaskFinishedResult::Success,
                    task_entity: parent,
                });
            }
        }
    }
    // if let Some(parent) = parents.parent(child) {
    //     println!("Adding observer to child for TaskFinished");
    //     commands.entity(child).observe(
    //         move |_trigger: Trigger<TaskFinished>,
    //               children: Query<&Children>,
    //               mut tasks: Query<&mut Task>,
    //               mut commands: Commands| {
    //             
    //             println!("Task finished triggered, checking if all children are finished");
    //             let all_parent_children = children.children(parent);
    //             
    //             let all_children_finished = all_parent_children.iter().all(|child| {
    //                 let task_data = tasks.get(*child).unwrap();
    //                 task_data.status == Status::Finished
    //             });
    // 
    //             if all_children_finished {
    //                 println!("All children finished, triggering parent finished");
    //                 let mut parent_task = tasks.get_mut(parent).unwrap();
    //                 parent_task.status = Status::Finished;
    //                 commands.entity(parent).trigger(TaskFinished(TaskFinishedResult::Success));
    //             }
    //         },
    //     );
    // }
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
            Some(TaskType::Build(build_data)) => score_build(build_data, agents, others_query),
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
                    let (_entity, child_task_data, sub_children) = all_tasks.get(&child).unwrap();
                    let next_sub_task =
                        get_available_task(child, task_data, *sub_children, all_tasks);
                    if let Some(next_sub_task) = next_sub_task {
                        let (_, next_sub_task_data, _) = all_tasks.get(&next_sub_task).unwrap();
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
                    let (_entity, _child_task_data, sub_children) = all_tasks.get(&child).unwrap();
                    let next_sub_task =
                        get_available_task(child, task_data, *sub_children, all_tasks);
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
