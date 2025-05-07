pub mod build;
pub mod deposit;
pub mod destruct;
pub mod escape_from_solid;
pub mod finish_task;
pub mod go_to;
pub mod pick_up;
pub mod play_animation;
pub mod fail_task;

use crate::features::ai::actions::build::build_action;
use crate::features::ai::actions::destruct::destruct_action;
use bevy::prelude::*;
use crate::features::ai::actions::finish_task::{clean_up_finished_tasks};

pub struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (build_action, destruct_action));
        app.add_systems(Update, clean_up_finished_tasks);
    }
}
