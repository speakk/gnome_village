use crate::ui::ui_main_actions::build_menu::BuildMenuBuildableSelected;
use crate::ui::ui_main_actions::orders_menu::OrderMenuItemSelected;
use bevy::app::{App, Plugin};
use bevy::prelude::{Commands, Entity, Event, ResMut, Resource};
use bevy_cobweb::prelude::{broadcast, BroadcastEvent, ReactCommandsExt};
use bevy_cobweb_ui::loading::scene_traits::SceneNodeBuilder;
use bevy_cobweb_ui::loading::SceneHandle;
use bevy_cobweb_ui::prelude::*;
use crate::ui::new_in_game::MainActionButtonType;

pub mod build_menu;
pub mod main_action_buttons;
pub mod orders_menu;

#[derive(Event)]
pub struct MainActionMenuButtonPressed(pub MainActionButtonType);

#[derive(Event)]
pub struct MainMenuSelectionCleared;

#[derive(Event)]
pub struct MainMenuSelected(pub MainActionButtonType);

#[derive(Resource)]
struct CurrentlySelectedMenu(Option<MainActionButtonType>);

pub struct MainActionsPlugin;

impl Plugin for MainActionsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource::<CurrentlySelectedMenu>(CurrentlySelectedMenu(None))
            .add_event::<MainActionMenuButtonPressed>()
            .add_event::<MainMenuSelectionCleared>()
            .add_event::<MainMenuSelected>()
            .add_event::<BuildMenuBuildableSelected>()
            .add_event::<OrderMenuItemSelected>();
    }
}

pub fn initialize_main_actions_menu<'a>(
    main_scene: &mut SceneHandle<'a, <UiBuilder<'_, Entity> as SceneNodeBuilder>::Builder<'a>>,
) {
    main_scene.get("action_menu_container").update_on(
        broadcast::<MainActionMenuButtonPressed>(),
        move |_id: UpdateId,
              event: BroadcastEvent<MainActionMenuButtonPressed>,
              mut currently_selected_menu: ResMut<CurrentlySelectedMenu>,
              mut commands: Commands| {
            if let Ok(event) = event.try_read() {
                commands.react().broadcast(MainMenuSelectionCleared);

                if Some(event.0) == currently_selected_menu.0 {
                    currently_selected_menu.0 = None;
                    return;
                }

                currently_selected_menu.0 = Some(event.0);
                commands.react().broadcast(MainMenuSelected(event.0));
            }
        },
    );
}
