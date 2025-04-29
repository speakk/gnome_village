use crate::features::states::AppState::InGame;
use crate::ui::ui_main_actions::MainActionMenuButtonPressed;
use crate::ui::widgets::CreateButton;
use bevy::ecs::spawn::SpawnWith;
use bevy::prelude::*;

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

fn main_action_buttons(mut commands: Commands) {
    let buttons = vec![
        MainActionButton {
            label: "Build".to_string(),
            main_action_type: MainActionButtonType::Build,
            index: 0,
        },
        MainActionButton {
            label: "Orders".to_string(),
            main_action_type: MainActionButtonType::Orders,
            index: 1,
        },
    ];

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            display: Display::Grid,
            grid_template_columns: vec![GridTrack::flex(1.0)],
            grid_template_rows: vec![GridTrack::flex(1.0), GridTrack::px(80.)],
            ..Default::default()
        },
        Pickable::IGNORE,
        children![
            // Top container
            (
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Auto,
                    ..Default::default()
                },
                Pickable::IGNORE,
                children![(
                    Node {
                        position_type: PositionType::Absolute,
                        bottom: Val::Px(0.0),
                        left: Val::Px(12.0),
                        ..Default::default()
                    },
                    MainActionMenuContainer,
                )]
            ),
            // Action buttons container
            (
                Node {
                    display: Display::Flex,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(12.0),
                    padding: UiRect::left(Val::Px(12.)),
                    ..Default::default()
                },
                Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
                    for button in buttons.iter().cloned() {
                        parent
                            .spawn((
                                Node {
                                    width: Val::Px(130.0),
                                    height: Val::Px(60.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    padding: UiRect::all(Val::Px(10.0)),
                                    ..default()
                                },
                                CreateButton {
                                    label: button.clone().label.clone(),
                                    font_size: 32.0,
                                    ..Default::default()
                                },
                            ))
                            .observe(
                                move |_trigger: Trigger<Pointer<Click>>,
                                      mut event_writer: EventWriter<
                                    MainActionMenuButtonPressed,
                                >| {
                                    event_writer.write(MainActionMenuButtonPressed(button.clone()));
                                },
                            );
                    }
                }))
            )
        ],
    ));
}
