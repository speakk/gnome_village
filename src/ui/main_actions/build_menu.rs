use std::cmp::PartialEq;
use bevy::prelude::*;
use bevy_cobweb::prelude::*;
use bevy_cobweb_ui::prelude::*;
use crate::ui::main_actions::{MainActionSelected, MainActionType};
use crate::ui::UiSceneHandles;

pub fn insert_build_menu(ui_scene_handles: Res<UiSceneHandles>, mut commands: Commands) {
    commands.
        ui_builder(ui_scene_handles.action_menu_container.unwrap()).
        update_on(broadcast::<MainActionSelected>(),
    |id: UpdateId, event: BroadcastEvent<MainActionSelected>, mut commands: Commands, mut scene_builder: ResMut<SceneBuilder>| {
        println!("In insert_build_menu thing!!");
        if let Ok(event) = event.try_read() {
            if event.0 != MainActionType::Build { return; }

            println!("Spawning build menu");
            commands.ui_builder(*id).spawn_scene(("build_menu", "build_menu"), &mut scene_builder);
        }
    });
}