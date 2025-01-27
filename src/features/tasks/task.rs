use moonshine_core::prelude::Save;
use bevy::prelude::Component;
use bevy::prelude::*;
use crate::bundles::ItemId;

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
pub struct ItemRequirement {
    pub item_id: ItemId,
    pub amount: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum DepositTarget {
    Coordinate(IVec2),
    Inventory(Entity),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct BringResourceData {
    pub item_requirement: ItemRequirement,
    pub target: DepositTarget
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum TaskType {
    Build,
    BringResource(BringResourceData),
    GoTo,
}

#[derive(Component, Debug, Clone, Copy, Reflect)]
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
