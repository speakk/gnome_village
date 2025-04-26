use crate::features::states::AppState::InGame;
use crate::ui::in_game::{MainActionButton, MainActionButtonType, MainActionMenuContainer};
use crate::ui::ui_main_actions::build_menu::{create_build_menu, BuildMenuBuildableSelected};
use crate::ui::ui_main_actions::orders_menu::{create_orders_menu, OrderMenuItemSelected};
use bevy::app::{App, Plugin, Update};
use bevy::prelude::*;
use bevy_easings::Ease;
use std::time::Duration;

pub mod build_menu;
pub mod orders_menu;

#[derive(Event)]
pub struct MainActionMenuButtonPressed(pub MainActionButton);

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
            (
                react_to_main_action_menu_button_pressed,
                create_build_menu,
                create_orders_menu,
            )
                .run_if(in_state(InGame)),
        );
    }
}

fn react_to_main_action_menu_button_pressed(
    mut event_reader: EventReader<MainActionMenuButtonPressed>,
    mut main_menu_selection_cleared: EventWriter<MainMenuSelectionCleared>,
    mut currently_selected_menu: ResMut<CurrentlySelectedMenu>,
    mut main_menu_selected: EventWriter<MainMenuSelected>,
    mut query: Query<(Entity, &mut Node), With<MainActionMenuContainer>>,
    mut commands: Commands,
) {
    for event in event_reader.read() {
        main_menu_selection_cleared.send(MainMenuSelectionCleared);

        {
            let Ok((entity, mut node)) = query.single_mut() else { return; };
            
            // TODO: This is just eyeballed and will break if we change button width or margins,
            // to position the button menu correctly above the button.
            const BUTTON_WIDTH_PLUS_SPACING: f32 = 140.0;
            let left_transform = event.0.index as f32 * BUTTON_WIDTH_PLUS_SPACING;
            node.margin = UiRect::bottom(Val::Px(-100.0)).with_left(Val::Px(left_transform));

            // Animate the menu in
            commands.entity(entity).insert(node.clone().ease_to(
                Node {
                    margin: UiRect::bottom(Val::Px(0.0)).with_left(Val::Px(left_transform)),
                    ..Default::default()
                },
                bevy_easings::EaseFunction::BounceOut,
                bevy_easings::EasingType::Once {
                    duration: Duration::from_millis(150),
                },
            ));
            commands.entity(entity).despawn_related::<Children>();
        }

        if Some(event.0.main_action_type) == currently_selected_menu.0 {
            currently_selected_menu.0 = None;
            return;
        }

        currently_selected_menu.0 = Some(event.0.main_action_type);
        main_menu_selected.send(MainMenuSelected(event.0.main_action_type));
    }
}
