pub mod build;
pub mod deposit;
pub mod escape_from_solid;
pub mod finish_task;
pub mod go_to;
pub mod pick_up;
pub mod play_animation;

use crate::features::ai::actions::build::build_action;
use bevy::prelude::*;

pub struct ActionsPlugin;

//ActionPlugin::<(GoToAction, PickUpAction, FinishTaskAction, DepositAction)>::default(),
//

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, build_action);
    }
}
