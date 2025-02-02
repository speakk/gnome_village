pub mod build_task;
pub mod assign_jobs;

use bevy::prelude::*;

#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct Job;