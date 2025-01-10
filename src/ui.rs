use crate::ui::main_actions::build_menu::insert_build_menu;
use crate::ui::main_actions::main_action_buttons::initialize_menu_buttons;
use crate::ui::main_actions::{MainActionMenuButtonPressed, MainActionType, MainActionsPlugin};
use bevy::prelude::*;
use bevy_cobweb::prelude::{broadcast, BroadcastEvent, ReactCommandsExt};
use bevy_cobweb_ui::loading::scene_traits::SceneNodeBuilder;
use bevy_cobweb_ui::prelude::*;

pub mod main_actions;

pub struct UiPlugin;

#[derive(Event)]
pub struct MainMenuSelectionCleared;

#[derive(Event)]
pub struct MainMenuSelected(pub MainActionType);

//pub struct UiSceneHandle(pub SceneHandle<'static, <UiBuilder<'_, Entity> as SceneNodeBuilder>::Builder<'static>>);
#[derive(Resource, Default)]
pub struct UiSceneHandles {
    pub main: Option<Entity>,
    pub action_menu_container: Option<Entity>,
}

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<MainMenuSelectionCleared>()
            .add_event::<MainMenuSelected>()
            .add_plugins(CobwebUiPlugin)
            .add_plugins(MainActionsPlugin)
            .insert_resource::<CurrentlySelectedMenu>(CurrentlySelectedMenu(None))
            .insert_resource::<UiSceneHandles>(UiSceneHandles::default())
            .load("ui_templates/manifest.cob")
            .add_systems(OnEnter(LoadState::Done), (build_ui, insert_build_menu));
    }
}

#[derive(Resource)]
struct CurrentlySelectedMenu(Option<MainActionType>);

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
            
            main_scene.get("action_menu_container").update_on(
                broadcast::<MainActionMenuButtonPressed>(),
                move |id: UpdateId,
                      event: BroadcastEvent<MainActionMenuButtonPressed>,
                      mut currently_selected_menu: ResMut<CurrentlySelectedMenu>,
                    mut commands: Commands| {
                    if let Ok(event) = event.try_read() {
                        commands.react().broadcast(MainMenuSelectionCleared);
                        
                        println!("MainActionMenuButton pressed {:?}", event.0);
                        if Some(event.0) == currently_selected_menu.0
                        {
                            println!("Clearing main menu");
                            currently_selected_menu.0 = None;
                            return;
                        }

                        currently_selected_menu.0 = Some(event.0);
                        commands.react().broadcast(MainMenuSelected(event.0));
                        println!("Setting main menu to {:?}", event.0);
                    }
                },
            );
            initialize_menu_buttons(main_scene);
        },
    );
}
