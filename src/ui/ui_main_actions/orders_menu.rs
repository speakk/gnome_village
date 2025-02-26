use crate::ui::ui_main_actions::main_action_buttons::MainActionButtonType;
use crate::ui::ui_main_actions::{MainMenuSelected, MainMenuSelectionCleared};
use crate::ui::UiSceneHandles;
use bevy::ecs::component::{ComponentId, Components};
use bevy::ecs::world::FilteredEntityMut;
use bevy::prelude::*;
use bevy_cobweb::prelude::*;
use bevy_cobweb_ui::prelude::*;
use crate::bundles::category_tags::Tree;

#[derive(Clone, Debug)]
pub enum OrderId {
    Destruct(Vec<ComponentId>),
}

#[derive(Clone, Debug)]
pub struct OrderItem {
    pub id: OrderId,
    pub name: String,
}

#[derive(Event)]
pub struct OrderMenuItemSelected(pub OrderId);

pub fn get_filtered_entities(mut world: &mut World, filters: Vec<ComponentId>) -> Vec<Entity> {
    //let filtered_entities = world.get
    let mut builder = QueryBuilder::<FilteredEntityMut>::new(&mut world);
    //builder.and(filters.iter().for_each(|id| { id }));
    filters.iter().for_each(|id| {
        builder.and(|builder| {
            builder.ref_id(*id);
        });
    });

    let mut query_state = builder.build();
    query_state.iter(&world).map(|e| e.id()).collect::<Vec<_>>()
}

#[derive(Resource, Default)]
pub struct OrderUiItems(pub Vec<OrderItem>);

pub fn setup_order_ui_items(mut order_ui_items: ResMut<OrderUiItems>, components: &Components) {
    order_ui_items.0 = vec![OrderItem {
        id: OrderId::Destruct(vec![components.component_id::<Tree>().unwrap()]),
        name: "Chop Trees".to_string(),
    }];
    
}

pub fn insert_orders_menu(
    ui_scene_handles: Res<UiSceneHandles>,
    mut commands: Commands,
) {

    commands
        .ui_builder(ui_scene_handles.action_menu_container.unwrap())
        .update_on(
            broadcast::<MainMenuSelected>(),
            |id: UpdateId,
             event: BroadcastEvent<MainMenuSelected>,
             mut commands: Commands,
             mut scene_builder: ResMut<SceneBuilder>,
             order_ui_items: Res<OrderUiItems>
            | {
                if let Ok(event) = event.try_read() {
                    if event.0 != MainActionButtonType::Orders {
                        return;
                    }

                    commands.ui_builder(*id).spawn_scene_and_edit(
                        ("orders_menu", "orders_menu"),
                        &mut scene_builder,
                        move |orders_menu_handle| {
                            for order_item in order_ui_items.0.clone() {
                                let name = order_item.name.clone();

                                orders_menu_handle.spawn_scene_and_edit(
                                    ("orders_menu", "order_item"),
                                    move |order_item_handle| {
                                        order_item_handle.get("label").update_text(name);
                                        order_item_handle.on_pressed(
                                            move |mut order_item_selected_writer: EventWriter<
                                                OrderMenuItemSelected,
                                            >| {
                                                println!("Build item pressed, broadcasting");
                                                order_item_selected_writer
                                                    .send(OrderMenuItemSelected(order_item.id.clone()));
                                            },
                                        );
                                    },
                                );
                            }
                            orders_menu_handle.despawn_on_broadcast::<MainMenuSelectionCleared>();
                        },
                    );
                }
            },
        );
}
