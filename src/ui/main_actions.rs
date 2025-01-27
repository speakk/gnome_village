use bevy::app::{App, Plugin};
use bevy::prelude::{Entity, Event};
use bevy_cobweb_ui::loading::scene_traits::SceneNodeBuilder;
use bevy_cobweb_ui::loading::SceneHandle;
use bevy_cobweb_ui::prelude::*;

pub mod main_action_buttons;
pub mod build_menu;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MainActionType {
    Build,
    Orders
}

#[derive(Event, Debug)]
pub struct MainActionMenuButtonPressed(pub MainActionType);

pub struct MainActionsPlugin;

impl Plugin for MainActionsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<MainActionMenuButtonPressed>();
    }
}