use crate::features::tasks::task::{Status, Task, TaskFinished };
use beet::prelude::*;
use bevy::prelude::*;

#[action(finish_task_action)]
#[derive(Component, Reflect)]
#[require(Name::new("FinishTaskAction"))]
pub struct FinishTaskAction {
    pub task: Entity,
    pub tree_root: Entity,
}

fn finish_task_action(
    trigger: Trigger<OnRun>,
    action: Query<&FinishTaskAction>,
    mut task_data: Query<&mut Task>,
    mut commands: Commands,
    mut event_writer: EventWriter<TaskFinished>,
) {
    let agent = trigger.origin;
    let task = action.get(trigger.action).unwrap().task;
    let tree_root = action.get(trigger.action).unwrap().tree_root;
    let mut task_data = task_data.get_mut(task).unwrap();

    // TODO: THREE mechanisms here for signifying finished, oh dear lord
    task_data.status = Status::Finished;
    println!(
        "Task finished by agent: {:?}, triggering TaskFinished for task: {:?}",
        agent, task
    );
    commands.entity(task).trigger(TaskFinished {
        task_entity: task,
    });

    event_writer.write(TaskFinished {
        task_entity: task,
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