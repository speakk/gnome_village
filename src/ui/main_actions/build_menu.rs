use bevy::prelude::{Commands, EventReader, Query, Res};
use bevy_cobweb_ui::prelude::*;
use crate::ui::main_actions::{MainActionSelected, MainActionType};
use crate::ui::UiSceneHandles;

pub fn insert_build_menu(mut event_reader: EventReader<MainActionSelected>, ui_scene_handles: Res<UiSceneHandles>, mut commands: Commands) {
    for event in event_reader.read() {
        if event.0 != MainActionType::Build { return; }

        commands.ui_builder(ui_scene_handles.action_menu_container).on

    }
}