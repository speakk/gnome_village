use crate::bundles::ItemCategory;
use crate::features::user_actions::CategoryFilter;
use crate::ui::colours::{THEME_4_400, THEME_4_600, THEME_4_DEFAULT};
use crate::ui::in_game::{MainActionButtonType, MainActionMenuContainer};
use crate::ui::ui_main_actions::{MainMenuSelected, MainMenuSelectionCleared};
use crate::ui::widgets::{ColorDefinition, CreateButtonParams, WidgetSystems};
use crate::ui::{UiSceneHandles, FONT_SMALL};
use bevy::prelude::*;

#[derive(Clone, Debug)]
pub enum OrderId {
    Destruct(CategoryFilter),
}

#[derive(Clone, Debug)]
pub struct OrderItem {
    pub id: OrderId,
    pub name: String,
}

#[derive(Event, Clone)]
pub(crate) struct OrderMenuItemSelected(pub OrderId);

#[derive(Resource, Default)]
pub struct OrderUiItems(pub Vec<OrderItem>);

pub fn setup_order_ui_items(mut order_ui_items: ResMut<OrderUiItems>) {
    order_ui_items.0 = vec![
        OrderItem {
            id: OrderId::Destruct(Some(vec![ItemCategory::Tree])),
            name: "Chop Trees".to_string(),
        },
        OrderItem {
            id: OrderId::Destruct(Some(vec![ItemCategory::Rocks])),
            name: "Mine rocks".to_string(),
        },
    ];
}

pub fn create_orders_menu(
    query: Query<Entity, With<MainActionMenuContainer>>,
    mut event: EventReader<MainMenuSelected>,
    mut commands: Commands,
    order_ui_items: Res<OrderUiItems>,
    widget_systems: Res<WidgetSystems>,
) {
    let button_widget_system = widget_systems.button;

    if let Some(event) = event.read().next() {
        if event.0 == MainActionButtonType::Orders {
            commands
                .entity(query.single())
                .with_children(|menu_container| {
                    let cloned_ui_items = order_ui_items.0.clone();

                    menu_container
                        .spawn(Node {
                            flex_direction: FlexDirection::Column,
                            ..Default::default()
                        })
                        .with_children(|menu_buttons| {
                            for order_item in cloned_ui_items.clone() {
                                let name = order_item.name.clone();

                                let writer_item_id = OrderMenuItemSelected(order_item.id.clone());

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
                                            OrderMenuItemSelected,
                                        >| {
                                            event_writer.send(writer_item_id.clone());
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