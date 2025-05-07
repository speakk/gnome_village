use crate::features::tasks::jobs::assign_jobs;
use crate::features::tasks::jobs::build_task::react_to_blueprints;
use crate::features::tasks::jobs::destruct_task::react_to_destruct_target;
use crate::features::tasks::jobs::water_plants::react_to_lacking_growth_requirements;
use crate::features::tasks::task::{propagate_failed_upwards, propagate_finished_upwards, tick_cooldown, TaskCancelled, TaskFinished};
use bevy::prelude::*;

pub struct TasksPlugin;

impl Plugin for TasksPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TaskFinished>()
            .add_event::<TaskCancelled>()
            .add_systems(Update, propagate_finished_upwards)
            .add_systems(FixedUpdate, tick_cooldown)
            .add_observer(react_to_lacking_growth_requirements)
            .add_observer(propagate_failed_upwards)
            .add_systems(Update, (react_to_blueprints, react_to_destruct_target))
            .add_systems(
                Update,
                assign_jobs::assign_jobs.run_if(assign_jobs::jobs_changed),
            );
    }
}
