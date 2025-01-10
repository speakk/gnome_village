use bevy::prelude::*;
use bevy_cobweb_ui::loading::scene_traits::SceneNodeBuilder;
use bevy_cobweb_ui::prelude::*;
use crate::ui::main_actions::main_action_buttons::initialize_menu_buttons;
use crate::ui::main_actions::{MainActionsPlugin};
use crate::ui::main_actions::build_menu::insert_build_menu;

pub mod main_actions;

pub struct UiPlugin;

//pub struct UiSceneHandle(pub SceneHandle<'static, <UiBuilder<'_, Entity> as SceneNodeBuilder>::Builder<'static>>);
#[derive(Resource, Default)]
pub struct UiSceneHandles {
    pub main: Option<Entity>,
    pub action_menu_container: Option<Entity>
}

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(CobwebUiPlugin)
            .add_plugins(MainActionsPlugin)
            .insert_resource::<UiSceneHandles>(UiSceneHandles::default())
            .load("ui_templates/manifest.cob")
            .add_systems(OnEnter(LoadState::Done), (build_ui, insert_build_menu));
    }
}

fn build_ui(
    mut commands: Commands,
    mut scene_loader: ResMut<SceneBuilder>,
    mut ui_scene_handles: ResMut<UiSceneHandles>
) {
    commands.ui_root().spawn_scene_and_edit(("main", "main_scene"), &mut scene_loader, |main_scene| {
        ui_scene_handles.main = Some(main_scene.id());
        ui_scene_handles.action_menu_container = Some(main_scene.get("action_menu_container").id());
        initialize_menu_buttons(main_scene);
    });
}