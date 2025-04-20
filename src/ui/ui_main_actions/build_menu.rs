use crate::bundles::buildables::Buildable;
use crate::bundles::{ItemId, Prototypes};
use crate::ui::new_in_game::{MainActionButtonType, MainActionMenuContainer};
use crate::ui::ui_main_actions::{
    MainActionMenuButtonPressed, MainMenuSelected, MainMenuSelectionCleared,
};
use crate::ui::widgets::{ColorDefinition, CreateButtonParams, WidgetSystems};
use crate::ui::{UiSceneHandles, FONT_SMALL};
use bevy::prelude::*;
use crate::ui::colours::{THEME_3_400, THEME_3_600, THEME_3_DEFAULT, THEME_4_400, THEME_4_600, THEME_4_DEFAULT};
// use bevy_cobweb::prelude::*;
// use bevy_cobweb_ui::prelude::*;

#[derive(Event, Clone, Copy)]
pub(crate) struct BuildMenuBuildableSelected(pub ItemId);

pub fn create_build_menu(
    query: Query<Entity, With<MainActionMenuContainer>>,
    mut event: EventReader<MainMenuSelected>,
    mut commands: Commands,
    buildables: Query<&Buildable>,
    names: Query<&Name>,
    prototypes: Res<Prototypes>,
    widget_systems: Res<WidgetSystems>
) {
    let button_widget_system = widget_systems.button;

    if let Some(event) = event.read().next() {
        if event.0 == MainActionButtonType::Build {
            commands
                .entity(query.single())
                .with_children(|menu_container| {
                    let cloned_prototypes = prototypes.0.clone();

                    menu_container
                        .spawn(Node {
                            flex_direction: FlexDirection::Column,
                            ..Default::default()
                        })
                        .with_children(|menu_buttons| {
                            for (item_id, prototype_entity) in cloned_prototypes {
                                if buildables.get(prototype_entity).is_err() {
                                    continue;
                                }

                                let name = names.get(prototype_entity).unwrap().to_owned();

                                let writer_item_id = BuildMenuBuildableSelected(item_id);
                                let button_entity = menu_buttons
                                    .spawn(Node {
                                        width: Val::Px(130.0),
                                        height: Val::Px(30.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        padding: UiRect::all(Val::Px(10.0)),
                                        ..default()
                                    })
                                    .observe(
                                        move |_trigger: Trigger<Pointer<Click>>,
                                              mut event_writer: EventWriter<
                                                  BuildMenuBuildableSelected,
                                        >| {
                                            event_writer.send(writer_item_id);
                                        },
                                    )
                                    .id();

                                menu_buttons.enqueue_command(move |world: &mut World| {
                                    let mut commands = world.commands();
                                    commands.run_system_with_input(
                                        button_widget_system,
                                        CreateButtonParams {
                                            label: name.parse().unwrap(),
                                            button_entity,
                                            font_size: 18.0,
                                            font: FONT_SMALL.parse().unwrap(),
                                            color_definition: ColorDefinition {
                                                normal: THEME_4_DEFAULT,
                                                hovered: THEME_4_600,
                                                pressed: THEME_4_400
                                            },
                                            ..Default::default()
                                        },
                                    );
                                });
                            }
                        });
                });
        }
    }
}

// pub fn insert_build_menu(ui_scene_handles: Res<UiSceneHandles>, mut commands: Commands) {
//     commands
//         .ui_builder(ui_scene_handles.action_menu_container.unwrap())
//         .update_on(
//             broadcast::<MainMenuSelected>(),
//             |id: UpdateId,
//              event: BroadcastEvent<MainMenuSelected>,
//              mut commands: Commands,
//              mut scene_builder: ResMut<SceneBuilder>,
//              buildables: Query<&Buildable>,
//              names: Query<&Name>,
//              prototypes: Res<Prototypes>| {
//                 println!("In insert_build_menu thing!!");
//                 if let Ok(event) = event.try_read() {
//                     if event.0 != MainActionButtonType::Build {
//                         return;
//                     }
//
//                     commands.ui_builder(*id).spawn_scene_and_edit(
//                         ("build_menu", "build_menu"),
//                         &mut scene_builder,
//                         move |build_benu_handle| {
//                             for (item_id, prototype_entity) in prototypes.0.clone() {
//                                 if buildables.get(prototype_entity).is_err() {
//                                     continue;
//                                 }
//
//                                 let name = names.get(prototype_entity).unwrap().to_owned();
//
//                                 build_benu_handle.spawn_scene_and_edit(
//                                     ("build_menu", "build_item"),
//                                     move |build_item_handle| {
//                                         build_item_handle.get("label").update_text(name);
//                                         build_item_handle.on_pressed(
//                                             move |mut buildable_selected_writer: EventWriter<
//                                                 BuildMenuBuildableSelected,
//                                             >| {
//                                                 println!("Build item pressed, broadcasting");
//                                                 buildable_selected_writer
//                                                     .send(BuildMenuBuildableSelected(item_id));
//                                             },
//                                         );
//                                     },
//                                 );
//                             }
//                             build_benu_handle.despawn_on_broadcast::<MainMenuSelectionCleared>();
//                         },
//                     );
//                 }
//             },
//         );
// }
