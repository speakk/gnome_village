use moonshine_core::prelude::Save;
use bevy::prelude::Component;
use bevy::prelude::*;
use crate::bundles::{Id, ItemId, Reservations, ResourceItem};
use crate::bundles::settler::Settler;
use crate::features::ai::trees::bring_resource::score_bring_resource;
use crate::features::misc_components::InWorld;
use crate::features::position::WorldPosition;

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
    pub run_time_data: Option<BringResourceRuntimeData>
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Reflect)]
pub enum TaskType {
    Build,
    BringResource(BringResourceData),
    GoTo,
}

#[derive(Component, Debug, Clone, Reflect)]
#[require(Save, Name(|| "Task"))]
#[reflect(Component)]
pub struct Task {
    pub run_type: RunType,
    pub status: Status,
    pub task_type: Option<TaskType>,
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
    pub fn find_best_agent(&mut self,
                       mut resources_query: &mut Query<(Entity, &WorldPosition, &Id, &mut Reservations), (With<ResourceItem>, With<InWorld>)>,
                           others_query: &Query<(Entity, &WorldPosition), (Without<ResourceItem>, Without<Settler>)>,
                       agents: &Vec<(Entity, &WorldPosition)>) -> Option<Entity> {
        match &mut self.task_type {
            Some(TaskType::BringResource(bring_resource_data)) => score_bring_resource(resources_query, agents, bring_resource_data, others_query),
            _ => None,
        }
    }
}