use crate::bundles::buildables::Buildable;
use crate::ui::ui_main_actions::main_action_buttons::MainActionButtonType;
use crate::ui::ui_main_actions::{MainMenuSelected, MainMenuSelectionCleared};
use crate::ui::UiSceneHandles;
use bevy::prelude::*;
use bevy_cobweb::prelude::*;
use bevy_cobweb_ui::prelude::*;

#[derive(Event)]
pub struct BuildMenuBuildableSelected(pub Entity);

pub fn insert_build_menu(ui_scene_handles: Res<UiSceneHandles>, mut commands: Commands) {
    commands
        .ui_builder(ui_scene_handles.action_menu_container.unwrap())
        .update_on(
            broadcast::<MainMenuSelected>(),
            |id: UpdateId,
             event: BroadcastEvent<MainMenuSelected>,
             mut commands: Commands,
             mut _scene_builder: ResMut<SceneBuilder>,
             buildables_query: Query<(Entity, &Name), With<Buildable>>| {
                println!("In insert_build_menu thing!!");
                if let Ok(event) = event.try_read() {
                    if event.0 != MainActionButtonType::Build {
                        return;
                    }

                    commands.ui_builder(*id).spawn_scene_and_edit(
                        ("build_menu", "build_menu"),
                        &mut _scene_builder,
                        move |build_benu_handle| {
                            for (entity, name) in buildables_query.iter() {
                                build_benu_handle.spawn_scene_and_edit(
                                    ("build_menu", "build_item"),
                                    move |build_item_handle| {
                                        build_item_handle.get("label").update_text(name);
                                        build_item_handle.on_pressed(
                                            move |mut buildable_selected_writer: EventWriter<
                                                BuildMenuBuildableSelected,
                                            >| {
                                                println!("Build item pressed, broadcasting");
                                                buildable_selected_writer
                                                    .send(BuildMenuBuildableSelected(entity));
                                            },
                                        );
                                    },
                                );
                            }
                            build_benu_handle.despawn_on_broadcast::<MainMenuSelectionCleared>();
                        },
                    );
                }
            },
        );
}
