use crate::ui::ui_main_actions::build_menu::insert_build_menu;
use crate::ui::ui_main_actions::main_action_buttons::initialize_menu_buttons;
use crate::ui::ui_main_actions::{initialize_main_actions_menu, MainActionsPlugin};
use bevy::prelude::*;
use bevy_cobweb_ui::prelude::*;

pub mod ui_main_actions;

pub struct UiPlugin;

#[derive(Resource, Default)]
pub struct UiSceneHandles {
    pub main: Option<Entity>,
    pub action_menu_container: Option<Entity>,
}

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CobwebUiPlugin)
            .add_plugins(MainActionsPlugin)
            .insert_resource::<UiSceneHandles>(UiSceneHandles::default())
            .load("ui_templates/manifest.cob")
            .add_systems(OnEnter(LoadState::Done), (build_ui, insert_build_menu));
    }
}

fn build_ui(
    mut commands: Commands,
    mut scene_loader: ResMut<SceneBuilder>,
    mut ui_scene_handles: ResMut<UiSceneHandles>,
) {
    commands.ui_root().spawn_scene_and_edit(
        ("main", "main_scene"),
        &mut scene_loader,
        |main_scene| {
            ui_scene_handles.main = Some(main_scene.id());
            ui_scene_handles.action_menu_container =
                Some(main_scene.get("action_menu_container").id());

            initialize_main_actions_menu(main_scene);
            initialize_menu_buttons(main_scene);
        },
    );
}
