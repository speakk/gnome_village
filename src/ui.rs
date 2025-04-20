use crate::ui::ui_main_actions::orders_menu::{
    setup_order_ui_items, OrderUiItems,
};
use crate::ui::ui_main_actions::{MainActionsPlugin};
use bevy::prelude::*;
use crate::features::states::AppState::InGame;
use crate::ui::day_cycle_indicator::DayCycleIndicatorPlugin;
use crate::ui::main_menu::MainMenuPlugin;
use crate::ui::in_game::new_in_game_plugin;
use crate::ui::widgets::widget_plugin;

pub mod ui_main_actions;
mod colours;
mod day_cycle_indicator;
mod main_menu;
mod in_game;
mod widgets;

pub struct UiPlugin;

pub static FONT_BOLD:&'static str = "fonts/ThaleahFat.ttf";
pub static FONT_SMALL:&'static str = "fonts/m5x7.ttf";

#[derive(Resource, Default)]
pub struct UiSceneHandles {
    pub main: Option<Entity>,
    pub action_menu_container: Option<Entity>,
}

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((new_in_game_plugin, widget_plugin))
            .add_plugins(MainMenuPlugin)
            .add_plugins(MainActionsPlugin)
            .add_plugins(DayCycleIndicatorPlugin)
            .insert_resource::<UiSceneHandles>(UiSceneHandles::default())
            .insert_resource::<OrderUiItems>(OrderUiItems::default())
            .add_systems(
                OnEnter(InGame),
                (
                    setup_order_ui_items,
                )
                    .chain(),
            );
    }
}