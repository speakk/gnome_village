use crate::features::states::AppState::InGame;
use crate::ui::in_game::{MainActionButtonType, MainActionMenuContainer};
use crate::ui::ui_main_actions::build_menu::{create_build_menu, BuildMenuBuildableSelected};
use crate::ui::ui_main_actions::orders_menu::{create_orders_menu, OrderMenuItemSelected};
use bevy::app::{App, Plugin, Update};
use bevy::prelude::*;

pub mod build_menu;
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

        app.add_systems(
            Update,
            (react_to_main_action_menu_button_pressed, create_build_menu, create_orders_menu).run_if(in_state(InGame)),
        );
    }
}

fn react_to_main_action_menu_button_pressed(
    mut event_reader: EventReader<MainActionMenuButtonPressed>,
    mut main_menu_selection_cleared: EventWriter<MainMenuSelectionCleared>,
    mut currently_selected_menu: ResMut<CurrentlySelectedMenu>,
    mut main_menu_selected: EventWriter<MainMenuSelected>,
    query: Query<Entity, With<MainActionMenuContainer>>,
    mut commands: Commands,
) {
    for event in event_reader.read() {
        main_menu_selection_cleared.send(MainMenuSelectionCleared);
        commands.entity(query.single()).despawn_descendants();

        if Some(event.0) == currently_selected_menu.0 {
            currently_selected_menu.0 = None;
            return;
        }

        currently_selected_menu.0 = Some(event.0);
        main_menu_selected.send(MainMenuSelected(event.0));
    }
}
