use crate::ui::ui_main_actions::{MainActionMenuButtonPressed};
use bevy::prelude::*;
use bevy_cobweb::prelude::ReactCommandsExt;
use bevy_cobweb_ui::loading::scene_traits::SceneNodeBuilder;
use bevy_cobweb_ui::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MainActionButtonType {
    Build,
    Orders,
}

struct MainActionButton {
    label: String,
    main_action_type: MainActionButtonType,
}

pub fn initialize_menu_buttons<'a>(
    main_scene: &mut SceneHandle<'a, <UiBuilder<'_, Entity> as SceneNodeBuilder>::Builder<'a>>,
) {
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

    for button in buttons {
        main_scene.get("buttons_container").spawn_scene_and_edit(
            ("button", "button"),
            |button_scene| {
                button_scene.get("text").update_text(&button.label);
                button_scene.on_pressed(move |mut commands: Commands| {
                    println!("Button pressed, broadcasting");
                    commands
                        .react()
                        .broadcast(MainActionMenuButtonPressed(button.main_action_type));
                });
            },
        );
    }
}
