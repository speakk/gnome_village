use crate::features::states::AppState::InGame;
use crate::ui::ui_main_actions::MainActionMenuButtonPressed;
use crate::ui::widgets::{CreateButtonParams, WidgetSystems};
use bevy::prelude::*;
use bevy::ui::AlignItems::FlexStart;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MainActionButtonType {
    Build,
    Orders,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(super) struct MainActionButton {
    pub(super) label: String,
    pub(super) main_action_type: MainActionButtonType,
    pub(super) index: usize, // TODO: Temporary index to position menu that pops up
}

// Markers start
#[derive(Component)]
pub struct MainActionMenuContainer;
// Markers end

pub(super) fn new_in_game_plugin(app: &mut App) {
    app.add_systems(OnEnter(InGame), main_action_buttons);
}

fn main_action_buttons(mut commands: Commands, widget_systems: Res<WidgetSystems>) {
    let buttons = [MainActionButton {
            label: "Build".to_string(),
            main_action_type: MainActionButtonType::Build,
            index: 0
        },
        MainActionButton {
            label: "Orders".to_string(),
            main_action_type: MainActionButtonType::Orders,
            index: 1
        }];

    let button_widget_system = widget_systems.button;

    let root_node = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Grid,
                grid_template_columns: vec![GridTrack::flex(1.0)],
                grid_template_rows: vec![
                    GridTrack::flex(1.0),
                    GridTrack::px(80.),
                ],
                ..Default::default()
            },
            PickingBehavior::IGNORE,
        ))
        .with_children(|root_node| {
            // Top container
            root_node.spawn((Node {
                width: Val::Percent(100.0),
                height: Val::Auto,
                ..Default::default()
            }, PickingBehavior::IGNORE))
                .with_children(|top_container| {
                    top_container.spawn((Node {
                        position_type: PositionType::Absolute,
                        bottom: Val::Px(0.0),
                        left: Val::Px(12.0),
                        ..Default::default()
                    }, MainActionMenuContainer));
                });

            // Action buttons container
            root_node
                .spawn((Node {
                    display: Display::Flex,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(12.0),
                    padding: UiRect::left(Val::Px(12.)),
                    ..Default::default()
                },))
                .with_children(|action_buttons_container| {
                    for button in buttons.iter().cloned() {
                        let button_click = button.clone();
                        let button_entity = action_buttons_container
                            .spawn(Node {
                                width: Val::Px(130.0),
                                height: Val::Px(60.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                padding: UiRect::all(Val::Px(10.0)),
                                ..default()
                            })
                            .observe(
                                move |_trigger: Trigger<Pointer<Click>>,
                                      mut event_writer: EventWriter<
                                    MainActionMenuButtonPressed,
                                >| {
                                    event_writer
                                        .send(MainActionMenuButtonPressed(button_click.clone()));
                                },
                            )
                            .id();

                        action_buttons_container.enqueue_command(move |world: &mut World| {
                            let mut commands = world.commands();
                            commands.run_system_with_input(
                                button_widget_system,
                                CreateButtonParams {
                                    label: button.clone().label.clone(),
                                    button_entity,
                                    font_size: 32.0,
                                    ..Default::default()
                                },
                            );
                        });

                    }
                });
        })
        .id();
}
