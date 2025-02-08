use beet::prelude::Action;
use bevy::prelude::{Component, Entity, Query, Reflect};
use crate::features::tasks::task::{Status, Task, TaskFinished, TaskFinishedResult};

#[derive(Component, Action, Reflect)]
#[require(Name(|| "FinishTaskAction"))]
#[observers(finish_task_action)]
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
    let agent = agents.get(trigger.entity()).unwrap().0;
    let task = action.get(trigger.entity()).unwrap().task;
    let tree_root = action.get(trigger.entity()).unwrap().tree_root;
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

    commands
        .entity(trigger.entity())
        .trigger(OnRunResult::success());

    commands.entity(tree_root).despawn_recursive();

}