use crate::features::states::AppState::InGame;
use crate::ui::ui_main_actions::MainActionMenuButtonPressed;
use crate::ui::widgets::{CreateButtonParams, WidgetSystems};
use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MainActionButtonType {
    Build,
    Orders,
}

struct MainActionButton {
    label: String,
    main_action_type: MainActionButtonType,
}

pub(super) fn new_in_game_plugin(app: &mut App) {
    app.add_systems(OnEnter(InGame), main_action_buttons);
}

fn main_action_buttons(mut commands: Commands, widget_systems: Res<WidgetSystems>) {
    let buttons = vec![
        MainActionButton {
            label: "Build".to_string(),
            main_action_type: MainActionButtonType::Build,
        },
        MainActionButton {
            label: "Orders".to_string(),
            main_action_type: MainActionButtonType::Orders,
        },
    ];

    let parent = commands
        .spawn((Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..Default::default()
        },))
        .id();

    for button in buttons {
        let button_entity = commands
            .spawn_empty()
            .observe(
                move |_trigger: Trigger<Pointer<Click>>,
                 mut event_writer: EventWriter<MainActionMenuButtonPressed>| {
                    event_writer.send(MainActionMenuButtonPressed(button.main_action_type));
                },
            ).id();
        commands.entity(parent).add_child(button_entity);
        commands.run_system_with_input(
            widget_systems.button,
            CreateButtonParams {
                label: button.label,
                button_entity,
            },
        );
    }
}
