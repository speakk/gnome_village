use bevy::app::{App, Plugin};
use bevy::prelude::{Commands, Entity, Event, ResMut, Resource};
use bevy_cobweb::prelude::{broadcast, BroadcastEvent, ReactCommandsExt};
use bevy_cobweb_ui::loading::scene_traits::SceneNodeBuilder;
use bevy_cobweb_ui::loading::SceneHandle;
use bevy_cobweb_ui::prelude::*;
use crate::ui::main_actions::build_menu::BuildMenuBuildableSelected;

pub mod main_action_buttons;
pub mod build_menu;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MainActionType {
    Build,
    Orders
}

#[derive(Event, Debug)]
pub struct MainActionMenuButtonPressed(pub MainActionType);

#[derive(Event)]
pub struct MainMenuSelectionCleared;

#[derive(Event)]
pub struct MainMenuSelected(pub MainActionType);


#[derive(Resource)]
struct CurrentlySelectedMenu(Option<MainActionType>);

pub struct MainActionsPlugin;

impl Plugin for MainActionsPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource::<CurrentlySelectedMenu>(CurrentlySelectedMenu(None))
            .add_event::<MainActionMenuButtonPressed>()
            .add_event::<MainMenuSelectionCleared>()
            .add_event::<MainMenuSelected>()
            .add_event::<BuildMenuBuildableSelected>();
    }
}

pub fn initialize_main_actions_menu<'a>(main_scene: &mut SceneHandle<'a, <UiBuilder<'_, Entity> as SceneNodeBuilder>::Builder<'a>>) {
    main_scene.get("action_menu_container").update_on(
        broadcast::<MainActionMenuButtonPressed>(),
        move |_id: UpdateId,
              event: BroadcastEvent<MainActionMenuButtonPressed>,
              mut currently_selected_menu: ResMut<CurrentlySelectedMenu>,
              mut commands: Commands| {
            if let Ok(event) = event.try_read() {
                commands.react().broadcast(MainMenuSelectionCleared);

                println!("MainActionMenuButton pressed {:?}", event.0);
                if Some(event.0) == currently_selected_menu.0
                {
                    println!("Clearing main menu");
                    currently_selected_menu.0 = None;
                    return;
                }

                currently_selected_menu.0 = Some(event.0);
                commands.react().broadcast(MainMenuSelected(event.0));
                println!("Setting main menu to {:?}", event.0);
            }
        },
    );
}