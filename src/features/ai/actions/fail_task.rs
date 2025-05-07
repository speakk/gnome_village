use crate::features::tasks::task::{Status, Task, TaskFailed, TaskFinished};
use beet::prelude::*;
use bevy::prelude::*;

#[action(fail_task_action)]
#[derive(Component, Reflect)]
#[require(Name::new("FailTaskAction"))]
pub struct FailTaskAction {
    pub task: Entity,
    pub tree_root: Entity,
}

fn fail_task_action(
    trigger: Trigger<OnRun>,
    action: Query<&FailTaskAction>,
    mut task_data: Query<&mut Task>,
    mut commands: Commands,
) {
    let agent = trigger.origin;
    let task = action.get(trigger.action).unwrap().task;
    let mut task_data = task_data.get_mut(task).unwrap();

    task_data.status = Status::Failed;
    println!(
        "Task failed by agent: {:?}, triggering TaskFailed for task: {:?}",
        agent, task
    );
    commands.entity(task).trigger(TaskFailed {
        reason: "Task failed, no reason implemented yet".to_string(),
    });

    trigger.trigger_result(&mut commands, RunResult::Success);
}

pub(super) fn clean_up_finished_tasks(
    mut commands: Commands,
    mut event: EventReader<TaskFinished>
) {
    for event in event.read() {
        let task = event.task_entity;
        commands.entity(task).despawn();
    }
}