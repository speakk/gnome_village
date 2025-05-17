use bevy::ecs::query::{QueryData, QueryFilter};
use bevy::prelude::*;
use crate::bundles::{Id, Reservations, ResourceItem};
use crate::features::misc_components::InWorld;
use crate::features::position::WorldPosition;

pub mod jobs;
pub mod task;
pub mod tasks_plugin;
pub mod sub_tasks;
