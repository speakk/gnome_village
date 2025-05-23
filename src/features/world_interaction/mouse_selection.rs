use crate::features::input::{
    world_interaction_action, InGameInputContext,
};
use crate::features::map::map_model::MapData;
use crate::features::states::AppState;
use crate::features::user_actions::{CurrentUserActionState, UserActionState};
use bevy::prelude::KeyCode::{ControlLeft, ShiftLeft};
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use bresenham::Bresenham;

#[derive(Event, Debug)]
pub struct CoordinatesSelectedEvent {
    pub coordinates: Vec<IVec2>,
    pub selection_type: SelectionType,
    pub drag_modifier: Option<DragModifier>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DragModifier {
    Primary,
    Secondary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SelectionType {
    Primary,
    Secondary,
}

#[derive(Event, Clone, Copy, Debug)]
pub struct MapDragStartEvent {
    pub selection_type: SelectionType,
    pub coordinate: IVec2,
    pub drag_modifier: Option<DragModifier>,
}

#[derive(Event)]
pub struct MapDragEndEvent {
    pub selection_type: SelectionType,
    pub coordinate: IVec2,
    pub drag_modifier: Option<DragModifier>,
}

pub struct MouseSelectionPlugin;

#[derive(Component)]
struct PickingGroundPlane;

impl Plugin for MouseSelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MeshPickingPlugin)
            .insert_resource(CurrentMouseWorldPosition(Vec2::ZERO))
            .insert_resource(CurrentMouseWorldCoordinate(IVec2::ZERO))
            .insert_resource(SelectedCoordinates::default())
            .insert_resource(DragInfo::default())
            .add_event::<CoordinatesSelectedEvent>()
            .add_event::<MapDragStartEvent>()
            .add_event::<MapDragEndEvent>()
            .add_observer(binding)
            .add_systems(OnEnter(AppState::InGame), setup)
            .add_systems(
                Update,
                (handle_mouse_dragged, scale_ground_mesh_based_on_map)
                    .run_if(in_state(AppState::InGame)),
            );
    }
}

fn binding(
    trigger: Trigger<Binding<InGameInputContext>>,
    mut input_context: Query<&mut Actions<InGameInputContext>>,
) {
    let mut actions = input_context.get_mut(trigger.target()).unwrap();

    actions
        .bind::<world_interaction_action::PrimaryDragModifier>()
        .to(ControlLeft);

    actions
        .bind::<world_interaction_action::SecondaryDragModifier>()
        .to(ShiftLeft);
}

fn setup(
    mut commands: Commands,
    mut mesh_picking_settings: ResMut<MeshPickingSettings>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    println!("Setting up mouse selection plugin");
    mesh_picking_settings.require_markers = true;
    mesh_picking_settings.ray_cast_visibility = RayCastVisibility::Any;

    let ground_plane_mesh = Mesh3d(
        meshes.add(
            Plane3d::default()
                .mesh()
                .size(200.0, 200.0)
                .subdivisions(10),
        ),
    );

    commands
        .spawn((
            PickingGroundPlane,
            ground_plane_mesh,
            Visibility::Hidden,
            MeshMaterial3d(materials.add(Color::srgb(0.5, 0.0, 0.0))),
            Transform::from_xyz(-0.5, 0.0, -0.5),
            Pickable::default(),
        ))
        .observe(handle_ground_plane_click)
        .observe(handle_ground_plane_hover)
        .observe(handle_ground_plane_drag_start)
        .observe(handle_ground_plane_drag_end);
}

fn scale_ground_mesh_based_on_map(
    mut query: Query<&mut Mesh3d, With<PickingGroundPlane>>,
    map_data_query: Query<&MapData, Changed<MapData>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let Ok(map_data) = map_data_query.get_single() else {
        return;
    };

    for mut mesh_3d in &mut query {
        let map_size = map_data.size;
        let new_handle = meshes.add(
            Plane3d::default()
                .mesh()
                .size(map_size.x as f32, map_size.y as f32)
                .subdivisions(10),
        );

        mesh_3d.0 = new_handle;
    }
}

fn handle_ground_plane_click(
    click: Trigger<Pointer<Click>>,
    mut coordinates_selected_event: EventWriter<CoordinatesSelectedEvent>,
) {
    // This is a workaround for a Bevy(?) bug which causes Click to trigger
    // on drag end as well
    if click.duration.as_secs_f32() > 0.2 {
        return;
    }

    let location = click.hit.position;
    if let Some(location) = location {
        coordinates_selected_event.write(CoordinatesSelectedEvent {
            coordinates: vec![IVec2::new(
                location.x.round() as i32,
                location.z.round() as i32,
            )],
            selection_type: if click.button == PointerButton::Primary {
                SelectionType::Primary
            } else {
                SelectionType::Secondary
            },
            drag_modifier: None,
        });
        println!("Clicked on location: {:?}", location);
    }
}

fn handle_ground_plane_drag_start(
    drag: Trigger<Pointer<DragStart>>,
    mut map_drag_start_event_writer: EventWriter<MapDragStartEvent>,
    mut drag_info_resource: ResMut<DragInfo>,
    input_context: Single<&Actions<InGameInputContext>>,
) {
    let actions = input_context.into_inner();
    let modifier_type = if actions
        .action::<world_interaction_action::PrimaryDragModifier>()
        .state()
        == ActionState::Fired
    {
        Some(DragModifier::Primary)
    } else if actions
        .action::<world_interaction_action::SecondaryDragModifier>()
        .state()
        == ActionState::Fired
    {
        Some(DragModifier::Secondary)
    } else {
        None
    };

    let location = drag.hit.position;
    if let Some(location) = location {
        println!(
            "Drag started on location: {:?} with modifier: {:?}",
            location, modifier_type
        );

        let event = MapDragStartEvent {
            coordinate: IVec2::new(location.x.round() as i32, location.z.round() as i32),
            drag_modifier: modifier_type,
            selection_type: if drag.button == PointerButton::Primary {
                SelectionType::Primary
            } else {
                SelectionType::Secondary
            },
        };

        map_drag_start_event_writer.write(event);

        println!("Drag start event written: {:?}", event);

        drag_info_resource.is_dragging = true;
        drag_info_resource.map_drag_start_event = Some(event);
    }
}

fn handle_ground_plane_drag_end(
    _drag: Trigger<Pointer<DragEnd>>,
    mut map_drag_end_event_writer: EventWriter<MapDragEndEvent>,
    mut coordinates_selected_event: EventWriter<CoordinatesSelectedEvent>,
    current_mouse_world_coordinate: Res<CurrentMouseWorldCoordinate>,
    mut selected_coordinates: ResMut<SelectedCoordinates>,
    mut drag_info_resource: ResMut<DragInfo>,
) {
    let location = current_mouse_world_coordinate.0;
    println!("Drag ended on location: {:?}", location);
    let drag_info_start_event = drag_info_resource
        .map_drag_start_event
        .expect("Drag start event not set when ending drag");
    map_drag_end_event_writer.write(MapDragEndEvent {
        coordinate: location,
        drag_modifier: drag_info_start_event.drag_modifier,
        selection_type: drag_info_start_event.selection_type,
    });
    drag_info_resource.is_dragging = false;
    drag_info_resource.map_drag_start_event = None;

    coordinates_selected_event.write(CoordinatesSelectedEvent {
        coordinates: selected_coordinates.0.clone(),
        selection_type: drag_info_start_event.selection_type,
        drag_modifier: drag_info_start_event.drag_modifier,
    });

    selected_coordinates.0.clear();
}

#[derive(Resource)]
pub struct CurrentMouseWorldPosition(pub Vec2);

#[derive(Resource)]
pub struct CurrentMouseWorldCoordinate(pub IVec2);

fn handle_ground_plane_hover(
    hover: Trigger<Pointer<Move>>,
    mut current_mouse_world_position: ResMut<CurrentMouseWorldPosition>,
    mut current_mouse_world_coordinate: ResMut<CurrentMouseWorldCoordinate>,
) {
    let location = hover.hit.position;
    if let Some(location) = location {
        //println!("Hovered on location: {:?}", location);
        current_mouse_world_position.0 = Vec2::new(location.x, location.z);

        let previous_mouse_coordinate = current_mouse_world_coordinate.0;
        let new_mouse_coordinate = IVec2::new(location.x.round() as i32, location.z.round() as i32);

        if previous_mouse_coordinate != new_mouse_coordinate {
            // println!(
            //     "Mouse coordinate changed from {:?} to {:?}",
            //     previous_mouse_coordinate, new_mouse_coordinate
            // );
            current_mouse_world_coordinate.0 = new_mouse_coordinate;
        }

        //println!("Setting current mouse world coordinate to: {:?}", current_mouse_world_coordinate.0);
    }
}

#[derive(Resource, Default, Copy, Clone, Debug)]
pub struct DragInfo {
    pub map_drag_start_event: Option<MapDragStartEvent>,
    pub is_dragging: bool,
}

pub fn handle_mouse_dragged(
    drag_info: Res<DragInfo>,
    mut selected_coordinates: ResMut<SelectedCoordinates>,
    current_coordinate: Res<CurrentMouseWorldCoordinate>,
    current_action_state: Res<CurrentUserActionState>,
) {
    if !current_coordinate.is_changed() {
        return;
    }

    if (drag_info.is_dragging) && (drag_info.map_drag_start_event.is_some()) {
        let Some(event) = drag_info.map_drag_start_event else {
            return;
        };

        match event.drag_modifier {
            Some(DragModifier::Primary) => {
                selected_coordinates.0 = line_select(event.coordinate, current_coordinate.0);
            }
            Some(DragModifier::Secondary) => {
                // TODO: Honestly this should always send all coordinates, or mix/max coordinates, and usage site should then what they need
                let hollow = matches!(
                    current_action_state.0,
                    Some(UserActionState::PlacingBuilding(_))
                ) || matches!(
                    current_action_state.0,
                    Some(UserActionState::CancellingJobs(_))
                );
                selected_coordinates.0 = rectangle_select(&current_coordinate, event, hollow);
            }
            None => {
                selected_coordinates.0 = vec![current_coordinate.0];
            }
        }
    } else {
        selected_coordinates.0 = vec![current_coordinate.0];
    }
}

#[derive(Resource, Default, Deref, DerefMut, Clone)]
pub struct SelectedCoordinates(pub Vec<IVec2>);

fn line_select(start_coordinate: IVec2, end_coordinate: IVec2) -> Vec<IVec2> {
    let mut line: Vec<_> = Bresenham::new(
        (start_coordinate.x as isize, start_coordinate.y as isize),
        (end_coordinate.x as isize, end_coordinate.y as isize),
    )
    .map(|point| IVec2::new(point.0 as i32, point.1 as i32))
    .collect();

    line.push(end_coordinate);

    line
}

fn rectangle_select(
    current_coordinate: &Res<CurrentMouseWorldCoordinate>,
    event: MapDragStartEvent,
    hollow: bool,
) -> Vec<IVec2> {
    let min_x = current_coordinate.0.x.min(event.coordinate.x);
    let min_y = current_coordinate.0.y.min(event.coordinate.y);
    let max_x = current_coordinate.0.x.max(event.coordinate.x);
    let max_y = current_coordinate.0.y.max(event.coordinate.y);
    let mut new_coordinates = Vec::new();
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            if hollow {
                if (x == min_x || x == max_x) || (y == min_y || y == max_y) {
                    new_coordinates.push(IVec2::new(x, y));
                }
            } else {
                new_coordinates.push(IVec2::new(x, y));
            }
        }
    }
    new_coordinates
}
