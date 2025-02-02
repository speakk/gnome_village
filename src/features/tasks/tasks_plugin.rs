use bevy::app::{App, Plugin, Update};
use bevy::prelude::IntoSystemConfigs;
use crate::features::tasks::jobs::assign_jobs;
use crate::features::tasks::jobs::build_task::react_to_blueprints;

pub struct TasksPlugin;

impl Plugin for TasksPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, react_to_blueprints)
            .add_systems(Update, assign_jobs::assign_jobs.run_if(assign_jobs::jobs_changed));
    }
}