use crate::bundles::buildables::BluePrint;
use crate::bundles::Id;
use crate::features::position::CoordinateToEntity;
use crate::features::user_actions::{UserActionIntent, UserActionType};
use crate::features::world_interaction::mouse_selection::{
    CoordinatesSelectedEvent, DragInfo, SelectedCoordinates, SelectionType,
};
use bevy::prelude::*;

pub struct CancelJobPlugin;

impl Plugin for CancelJobPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                draw_cancel_gizmo,
                send_cancel_intent,
                react_to_cancel_intent,
            ),
        );
    }
}

fn draw_cancel_gizmo(
    drag_info: Res<DragInfo>,
    selected_coordinates: Res<SelectedCoordinates>,
    mut gizmos: Gizmos,
) {
    if !drag_info.is_dragging {
        return;
    }

    if selected_coordinates.0.is_empty() {
        return;
    }

    let drag_start_event = drag_info
        .map_drag_start_event
        .expect("Drag start event not set when dragging");
    if drag_start_event.selection_type != SelectionType::Secondary {
        return;
    }

    for coordinate in selected_coordinates.0.iter() {
        gizmos.rounded_rect(
            Vec3::new(coordinate.x as f32, 0.1, coordinate.y as f32),
            Vec2::splat(1.0),
            Srgba::hex("#a84832").unwrap(),
        );
    }
}

fn send_cancel_intent(
    mut coordinated_selected_events: EventReader<CoordinatesSelectedEvent>,
    mut user_action_intent: EventWriter<UserActionIntent>,
) {
    let Some(event) = coordinated_selected_events.read().last() else {
        return;
    };

    if event.selection_type != SelectionType::Secondary {
        return;
    }

    user_action_intent.send(UserActionIntent(UserActionType::CancelJobs {
        coordinates: event.coordinates.clone(),
        id_filter: None,
    }));
}

pub fn react_to_cancel_intent(
    mut user_action_intent: EventReader<UserActionIntent>,
    item_ids: Query<&Id>,
    blueprints: Query<&BluePrint>,
    mut commands: Commands,
    coordinate_to_entity: Res<CoordinateToEntity>,
) {
    for event in user_action_intent.read() {
        if let UserActionType::CancelJobs {
            id_filter,
            coordinates,
        } = &event.0
        {
            println!(
                "Got cancel jobs intent with id filter: {:?} for coords: {:?}",
                id_filter, coordinates
            );

            for coordinate in coordinates.iter() {
                println!("Looking for entities within radius of: {:?}", coordinate);

                let entities = coordinate_to_entity.0.get(coordinate);

                if let Some(entities) = entities {
                    for entity in entities {
                        let entity_id = item_ids.get(*entity).unwrap();
                        println!("Entity id: {:?}", entity_id);

                        if let Ok(blueprint) = blueprints.get(*entity) {
                            commands.entity(*entity).despawn();
                        }
                    }
                }
            }
        }
    }
}
