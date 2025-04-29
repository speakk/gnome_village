use crate::bundles::buildables::Buildable;
use crate::bundles::{ItemId, Prototypes};
use crate::ui::colours::{THEME_4_400, THEME_4_600, THEME_4_DEFAULT};
use crate::ui::in_game::{MainActionButtonType, MainActionMenuContainer};
use crate::ui::ui_main_actions::MainMenuSelected;
use crate::ui::widgets::{ColorDefinition, CreateButton};
use crate::ui::FONT_SMALL;
use bevy::ecs::spawn::SpawnWith;
use bevy::prelude::*;
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
) {
    if let Some(event) = event.read().next() {
        if event.0 == MainActionButtonType::Build {
            commands
                .entity(query.single().unwrap())
                .with_children(move |menu_container| {
                    let mut buildable_items = Vec::new();
                    for (item_id, prototype_entity) in prototypes.0.iter() {
                        if let Ok(_) = buildables.get(*prototype_entity) {
                            if let Ok(name) = names.get(*prototype_entity) {
                                buildable_items.push((*item_id, name.to_string()));
                            }
                        }
                    }

                    menu_container.spawn((
                        Node {
                            flex_direction: FlexDirection::Column,
                            ..Default::default()
                        },
                        Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
                            for (item_id, name) in buildable_items {
                                let writer_item_id = BuildMenuBuildableSelected(item_id);
                                parent
                                    .spawn((
                                        Node {
                                            width: Val::Px(130.0),
                                            height: Val::Px(30.0),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            padding: UiRect::all(Val::Px(10.0)),
                                            ..default()
                                        },
                                        CreateButton {
                                            label: name.parse().unwrap(),
                                            font_size: 18.0,
                                            font: FONT_SMALL.parse().unwrap(),
                                            color_definition: ColorDefinition {
                                                normal: THEME_4_DEFAULT,
                                                hovered: THEME_4_600,
                                                pressed: THEME_4_400,
                                            },
                                        },
                                    ))
                                    .observe(
                                        move |_trigger: Trigger<Pointer<Click>>,
                                              mut event_writer: EventWriter<
                                            BuildMenuBuildableSelected,
                                        >| {
                                            event_writer.send(writer_item_id);
                                        },
                                    );
                            }
                        })),
                    ));
                });
        }
    }
}
