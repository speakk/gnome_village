use bevy::prelude::*;
use bevy_cobweb_ui::loading::scene_traits::SceneNodeBuilder;
use bevy_cobweb_ui::prelude::*;
use crate::ui::main_actions::{MainActionSelected, MainActionType};

struct MainActionButton {
    label: String,
    main_action_type: MainActionType
}


pub fn initialize_menu_buttons<'a>(main_scene: &mut SceneHandle<'a, <UiBuilder<'_, Entity> as SceneNodeBuilder>::Builder<'a>>) {
    let buttons = vec![
        MainActionButton {
            label: "Build".to_string(),
            main_action_type: MainActionType::Build
        },
        MainActionButton {
            label: "Orders".to_string(),
            main_action_type: MainActionType::Orders
        },
    ];

    for button in buttons {
        main_scene.get("buttons_container").spawn_scene_and_edit(("button", "button"), |button_scene| {
            button_scene.get("text").update_text(&button.label);
            button_scene.on_pressed(move |mut event_writer: EventWriter<MainActionSelected>| {
               event_writer.send(MainActionSelected(button.main_action_type));
                OK
            });
        });
    }
}