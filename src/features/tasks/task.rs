use bevy::prelude::Component;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RunType {
    Sequence,
    Parallel,
    Leaf
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Status {
    Ready,
    BeingWorkedOn,
    Finished,
    Failed,
    Cancelled
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TaskType {
    Build,
    BringResource,
    GoTo
}


#[derive(Component, Debug, Clone, Copy)]
pub struct Task {
    pub run_type: RunType,
    pub status: Status,
    pub task_type: Option<TaskType>
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