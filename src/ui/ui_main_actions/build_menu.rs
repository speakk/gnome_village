use crate::bundles::buildables::Buildable;
use crate::bundles::{ItemId, Prototypes};
use crate::ui::colours::{
    THEME_4_400, THEME_4_600, THEME_4_DEFAULT,
};
use crate::ui::new_in_game::{MainActionButtonType, MainActionMenuContainer};
use crate::ui::ui_main_actions::MainMenuSelected;
use crate::ui::widgets::{ColorDefinition, CreateButtonParams, WidgetSystems};
use crate::ui::FONT_SMALL;
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
    widget_systems: Res<WidgetSystems>,
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
                                                pressed: THEME_4_400,
                                            },
                                        },
                                    );
                                });
                            }
                        });
                });
        }
    }
}