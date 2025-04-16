use crate::ui::ui_main_actions::build_menu::insert_build_menu;
use crate::ui::ui_main_actions::orders_menu::{
    insert_orders_menu, setup_order_ui_items, OrderUiItems,
};
use crate::ui::ui_main_actions::{MainActionsPlugin};
use bevy::prelude::*;
use bevy_cobweb_ui::prelude::*;
use crate::features::states::AppState::InGame;
use crate::ui::day_cycle_indicator::DayCycleIndicatorPlugin;
use crate::ui::main_menu::MainMenuPlugin;
use crate::ui::new_in_game::new_in_game_plugin;
use crate::ui::widgets::widget_plugin;

pub mod ui_main_actions;
mod colours;
mod day_cycle_indicator;
mod main_menu;
mod new_in_game;
mod widgets;

pub struct UiPlugin;

#[derive(Resource, Default)]
pub struct UiSceneHandles {
    pub main: Option<Entity>,
    pub action_menu_container: Option<Entity>,
}

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CobwebUiPlugin)
            .add_plugins((new_in_game_plugin, widget_plugin))
            .add_plugins(MainMenuPlugin)
            .add_plugins(MainActionsPlugin)
            .add_plugins(DayCycleIndicatorPlugin)
            .insert_resource::<UiSceneHandles>(UiSceneHandles::default())
            .insert_resource::<OrderUiItems>(OrderUiItems::default())
            //.load("ui_templates/manifest.cob")
            .add_systems(
                OnEnter(InGame),
                (
                    setup_order_ui_items,
                    //build_ui,
                    //insert_build_menu,
                    //insert_orders_menu,
                )
                    .chain(),
            );
    }
}
// 
// fn build_ui(
//     mut commands: Commands,
//     mut scene_loader: ResMut<SceneBuilder>,
//     mut ui_scene_handles: ResMut<UiSceneHandles>,
// ) {
//     commands.ui_root().spawn_scene_and_edit(
//         ("main", "main_scene"),
//         &mut scene_loader,
//         |main_scene| {
//             ui_scene_handles.main = Some(main_scene.id());
//             ui_scene_handles.action_menu_container =
//                 Some(main_scene.get("action_menu_container").id());
// 
//             initialize_main_actions_menu(main_scene);
//             initialize_menu_buttons(main_scene);
//         },
//     );
// }
