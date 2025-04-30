use crate::bundles::buildables::BluePrint;
use crate::bundles::Id;
use crate::features::position::CoordinateToEntity;
use crate::features::user_actions::{
    CurrentUserActionState, UserActionIntent, UserActionState, UserActionType,
};
use crate::features::world_interaction::destruct_action::draw_rectangle_selection;
use crate::features::world_interaction::mouse_selection::{
    CoordinatesSelectedEvent, DragInfo, SelectedCoordinates, SelectionType,
};
use bevy::prelude::*;
use moonshine_view::Viewable;
use crate::features::misc_components::gltf_asset::GltfData;
use crate::features::misc_components::simple_mesh::SimpleMesh;

pub struct CancelJobPlugin;

impl Plugin for CancelJobPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                engage_cancelling_state,
                draw_cancel_gizmo,
                send_cancel_intent,
                react_to_cancel_intent,
            ),
        );
    }
}

fn engage_cancelling_state(mut current_user_action: ResMut<CurrentUserActionState>,
                           drag_info: Res<DragInfo>,) {
    let Some(map_drag_start_event) = drag_info.map_drag_start_event else {
        return;
    };

    if !matches!(current_user_action.0, Some(UserActionState::PlacingBuilding(_))) {
        return;
    }
    
    if map_drag_start_event.selection_type == SelectionType::Secondary {
        current_user_action.0 = Some(UserActionState::CancellingJobs(None));
    }
}

fn draw_cancel_gizmo(
    current_user_action: Res<CurrentUserActionState>,
    drag_info: Res<DragInfo>,
    selected_coordinates: Res<SelectedCoordinates>,
    mut gizmos: Gizmos,
) {
    if drag_info.map_drag_start_event.is_none() {
        return;
    };

    if !matches!(current_user_action.0, Some(UserActionState::CancellingJobs(_))) {
        return;
    }

    if selected_coordinates.0.is_empty() {
        return;
    }

    draw_rectangle_selection(selected_coordinates, &mut gizmos);
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

    user_action_intent.write(UserActionIntent(UserActionType::CancelJobs {
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
    // views: Query<&Viewable<SimpleMeshValid>>,
    // views_gltf: Query<&Viewable<GltfData>>
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

                        if blueprints.contains(*entity) {
                            // if let Ok(view) = views.get(*entity) {
                            //     commands.entity(view.view().entity()).despawn();
                            // }
                            // 
                            // if let Ok(view) = views_gltf.get(*entity) {
                            //     commands.entity(view.view().entity()).despawn();
                            // }
                            commands.entity(*entity).despawn();
                        }
                    }
                }
            }
        }
    }
}
