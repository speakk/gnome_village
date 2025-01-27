use std::cmp::PartialEq;
use bevy::prelude::*;
use bevy_cobweb::prelude::*;
use bevy_cobweb_ui::prelude::*;
use crate::bundles::buildables::Buildable;
use crate::ui::main_actions::{MainActionType};
use crate::ui::{MainMenuSelected, MainMenuSelectionCleared, UiSceneHandles};

pub fn insert_build_menu(ui_scene_handles: Res<UiSceneHandles>, mut commands: Commands) {
    commands.
        ui_builder(ui_scene_handles.action_menu_container.unwrap()).
        update_on(broadcast::<MainMenuSelected>(),
                  |id: UpdateId, event: BroadcastEvent<MainMenuSelected>, mut commands: Commands, mut _scene_builder: ResMut<SceneBuilder>, buildables_query: Query<&Name, With<Buildable>>| {
        println!("In insert_build_menu thing!!");
        if let Ok(event) = event.try_read() {
            if event.0 != MainActionType::Build { return; }

            println!("Spawning build menu");
            commands.ui_builder(*id).spawn_scene_and_edit(("build_menu", "build_menu"), &mut _scene_builder, move |build_benu_handle| {
                for name in buildables_query.iter() {
                    println!("Adding buildable: {}", name);
                    build_benu_handle.spawn_scene_and_edit(("build_menu", "build_item"), move |build_item_handle| {
                        build_item_handle.get("label").update_text(name);
                    });
                }
                build_benu_handle.despawn_on_broadcast::<MainMenuSelectionCleared>();
            });
        }
    });
}