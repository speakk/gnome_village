pub mod assign_jobs;
pub mod build_task;

use bevy::prelude::*;

#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct Job;
