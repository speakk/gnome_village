use beet::prelude::*;
use bevy::prelude::*;
use crate::features::ai::TargetEntity;
use crate::features::tasks::task::{Status, Task, TaskFinished, TaskFinishedResult};

#[action(finish_task_action)]
#[derive(Component, Reflect)]
#[require(Name(|| "FinishTaskAction"))]
pub struct FinishTaskAction {
    pub task: Entity,
    pub tree_root: Entity,
}

fn finish_task_action(
    trigger: Trigger<OnRun>,
    agents: Query<&TargetEntity>,
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
    println!("Task finished by agent: {:?}, triggering TaskFinished for task: {:?}", agent, task);
    commands.entity(task).trigger(TaskFinished {
        result: TaskFinishedResult::Success,
        task_entity: task,
    });

    event_writer.send(TaskFinished {
        result: TaskFinishedResult::Success,
        task_entity: task,
    });

    trigger.trigger_result(&mut commands, RunResult::Success);
    commands.entity(tree_root).despawn_recursive();
}