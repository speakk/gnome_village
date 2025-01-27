use crate::bundles::buildables::{Buildable, BuildableBundleTypes};
use crate::ui::ui_main_actions::main_action_buttons::MainActionButtonType;
use crate::ui::ui_main_actions::{MainMenuSelected, MainMenuSelectionCleared};
use crate::ui::UiSceneHandles;
use bevy::prelude::*;
use bevy_cobweb::prelude::*;
use bevy_cobweb_ui::prelude::*;
use crate::bundles::{ItemId, Prototypes};

#[derive(Event)]
pub struct BuildMenuBuildableSelected(pub ItemId);

pub fn insert_build_menu(ui_scene_handles: Res<UiSceneHandles>, mut commands: Commands) {
    commands
        .ui_builder(ui_scene_handles.action_menu_container.unwrap())
        .update_on(
            broadcast::<MainMenuSelected>(),
            |id: UpdateId,
             event: BroadcastEvent<MainMenuSelected>,
             mut commands: Commands,
             mut _scene_builder: ResMut<SceneBuilder>,
                buildables: Query<&Buildable>,
                names: Query<&Name, With<Buildable>>,
             prototypes: Res<Prototypes>| {
                println!("In insert_build_menu thing!!");
                if let Ok(event) = event.try_read() {
                    if event.0 != MainActionButtonType::Build {
                        return;
                    }

                    commands.ui_builder(*id).spawn_scene_and_edit(
                        ("build_menu", "build_menu"),
                        &mut _scene_builder,
                        move |build_benu_handle| {
                            for (item_id, prototype_entity) in prototypes.0.clone() {
                                if buildables.get(prototype_entity).is_err() {
                                    continue;
                                }

                                let name = names.get(prototype_entity).unwrap().to_owned();

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
                                                    .send(BuildMenuBuildableSelected(item_id));
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
