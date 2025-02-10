use bevy::color::Color::Oklaba;
use crate::features::world_interaction::mouse_selection::{CoordinatesSelectedEvent, DragInfo, SelectedCoordinates, SelectionType};
use bevy::color::palettes::basic::GREEN;
use bevy::prelude::*;
use crate::features::user_actions::{UserActionIntent, UserActionType};

pub struct CancelJobPlugin;

impl Plugin for CancelJobPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (draw_cancel_gizmo, send_cancel_intent, react_to_cancel_intent));
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

    let drag_start_event = drag_info
        .map_drag_start_event
        .expect("Drag start event not set when dragging");
    if drag_start_event.selection_type != SelectionType::Secondary {
        return;
    }

    let min_x = selected_coordinates
        .0
        .iter()
        .map(|coordinate| coordinate.x)
        .min()
        .unwrap();

    let max_x = selected_coordinates
        .0
        .iter()
        .map(|coordinate| coordinate.x)
        .max()
        .unwrap();

    let min_y = selected_coordinates
        .0
        .iter()
        .map(|coordinate| coordinate.y)
        .min()
        .unwrap();

    let max_y = selected_coordinates
        .0
        .iter()
        .map(|coordinate| coordinate.y)
        .max()
        .unwrap();

    // Ensure width and height are never zero
    let width = (max_x - min_x + 1).max(1); // +1 ensures inclusive bounds
    let height = (max_y - min_y + 1).max(1); // +1 ensures inclusive bounds

    // Calculate the center of the selection (correctly handles width/height adjustment)
    let center_x = min_x as f32 + (width as f32 - 1.0) / 2.0;
    let center_y = min_y as f32 + (height as f32 - 1.0) / 2.0;

    // Convert to world coordinates (assuming 0.5 is correct scaled height)
    let selected_coordinate_center_world = Vec3::new(center_x, 0.1, center_y);

    // Pass the center and the calculated width/height to `grid_3d`
    gizmos
        .grid(
            Isometry3d::from(selected_coordinate_center_world),
            UVec2::new(width as u32, height as u32),
            Vec2::splat(1.0),
            Srgba::hex("#a84832").unwrap()
        )
        .outer_edges();
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
) {
    for event in user_action_intent.read() {
        if let UserActionType::CancelJobs { id_filter, coordinates } = &event.0 {
            println!("Got cancel jobs intent with id filter: {:?} for coords: {:?}", id_filter, coordinates);
        }
    }
}