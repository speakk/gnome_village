use bevy::prelude::*;
use crate::bundles::{Id, ItemCategories};
use crate::features::position::CoordinateToEntity;
use crate::features::user_actions::{CurrentUserActionState, UserActionIntent, UserActionState, UserActionType};
use crate::features::world_interaction::mouse_selection::{CoordinatesSelectedEvent, DragInfo, SelectedCoordinates};

pub struct DestructActionPlugin;

#[derive(Component, Debug, Clone, Default, Reflect)]
#[reflect(Component)]
pub struct DestructTarget;

impl Plugin for DestructActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (draw_destruct_gizmo, send_destruct_intent, react_to_destruct_intent));
    }
}

fn draw_destruct_gizmo(
    current_user_action: Res<CurrentUserActionState>,
    drag_info: Res<DragInfo>,
    selected_coordinates: Res<SelectedCoordinates>,
    mut gizmos: Gizmos,
) {
    if drag_info.map_drag_start_event.is_none() {
        return;
    };

    if !matches!(current_user_action.0, UserActionState::Destructing(_)) {
        return
    }

    if selected_coordinates.0.is_empty() {
        return;
    }

    draw_rectangle_selection(selected_coordinates, &mut gizmos);
}

fn send_destruct_intent(
    mut coordinated_selected_events: EventReader<CoordinatesSelectedEvent>,
    mut user_action_intent: EventWriter<UserActionIntent>,
    current_user_action: Res<CurrentUserActionState>,
) {
    let Some(event) = coordinated_selected_events.read().last() else {
        return;
    };
    
    if let UserActionState::Destructing(category_id_filter) = &current_user_action.0 {
        user_action_intent.send(UserActionIntent(UserActionType::Destruct {
            coordinates: event.coordinates.clone(),
            category_id_filter: category_id_filter.clone(),
        }));
    }
}

pub fn react_to_destruct_intent(
    mut user_action_intent: EventReader<UserActionIntent>,
    item_ids: Query<&Id>,
    item_categories: Res<ItemCategories>,
    mut commands: Commands,
    coordinate_to_entity: Res<CoordinateToEntity>,
) {
    for event in user_action_intent.read() {
        if let UserActionType::Destruct {
            coordinates,
            category_id_filter,
        } = &event.0
        {
            println!(
                "Got destruct jobs intent with category id filter: {:?} for coords: {:?}",
                category_id_filter, coordinates
            );

            for coordinate in coordinates.iter() {
                let entities = coordinate_to_entity.0.get(coordinate);

                if let Some(entities) = entities {
                    for entity in entities {
                        let id_data = item_ids.get(*entity).unwrap();
                        let mut matches_category = false;
                        if let Some(category_id_filter) = category_id_filter {
                            for category_id in category_id_filter.iter() {
                                let ids_in_category = item_categories.0.get(category_id);
                                if let Some(ids_in_category) = ids_in_category {
                                    if ids_in_category.iter().collect::<Vec<_>>().contains(&&id_data.0) {
                                        matches_category = true;
                                        break;
                                    }
                                }

                            }
                        } else {
                            matches_category = true;
                        }

                        if matches_category {
                            commands.entity(*entity).insert(DestructTarget);
                        }
                    }
                }
            }
        }
    }
}


pub fn draw_rectangle_selection(selected_coordinates: Res<SelectedCoordinates>, gizmos: &mut Gizmos) {
    for coordinate in selected_coordinates.0.iter() {
        gizmos.rounded_rect(
            Vec3::new(coordinate.x as f32, 0.1, coordinate.y as f32),
            Vec2::splat(1.0),
            Srgba::hex("#a84832").unwrap(),
        );
    }
}
