use crate::features::input::WorldInteractionAction;
use crate::features::map::map_model::{MapData, ReservedCoordinatesHelper};
use crate::features::states::AppState;
use bevy::prelude::KeyCode::{ControlLeft, KeyA, KeyD, ShiftLeft};
use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;
use leafwing_input_manager::input_map::InputMap;
use leafwing_input_manager::InputManagerBundle;
use bresenham::Bresenham;
use crate::features::world_interaction::build_action;

#[derive(Event)]
pub struct MapClickedEvent(pub IVec2);

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

#[derive(Event, Clone, Copy)]
pub struct MapDragStartEvent {
    pub selection_type: SelectionType,
    pub coordinate: IVec2,
    pub drag_modifier: Option<DragModifier>,
}

#[derive(Event)]
pub struct MapDragEndEvent(pub IVec2);

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
            .add_event::<MapClickedEvent>()
            .add_event::<MapDragStartEvent>()
            .add_event::<MapDragEndEvent>()
            .add_systems(OnEnter(AppState::InGame), setup)
            .add_systems(
                Update,
                (handle_mouse_dragged, scale_ground_mesh_based_on_map).run_if(in_state(AppState::InGame)),
            );
    }
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

    let drag_input_map = InputMap::new([
        (WorldInteractionAction::PrimarySelect, KeyA),
        (WorldInteractionAction::SecondarySelect, KeyD),
        (WorldInteractionAction::PrimaryDragModifier, ControlLeft),
        (WorldInteractionAction::SecondaryDragModifier, ShiftLeft),
    ]);

    commands.spawn(InputManagerBundle::with_map(drag_input_map));

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
            RayCastPickable,
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
    mut map_clicked_event_writer: EventWriter<MapClickedEvent>,
    reserved_coordinates: Res<ReservedCoordinatesHelper>,
) {
    // This is a workaround for a Bevy(?) bug which causes Click to trigger
    // on drag end as well
    if click.duration.as_secs_f32() > 0.2 {
        return;
    }
    
    let location = click.hit.position;
    if let Some(location) = location {
        println!("Clicked on location: {:?}", location);
        map_clicked_event_writer.send(MapClickedEvent(IVec2::new(
            location.x as i32,
            location.z as i32,
        )));
    }
}

fn handle_ground_plane_drag_start(
    drag: Trigger<Pointer<DragStart>>,
    mut map_drag_start_event_writer: EventWriter<MapDragStartEvent>,
    mut drag_info_resource: ResMut<DragInfo>,
    interaction_action_query: Query<(&ActionState<WorldInteractionAction>,)>,
) {
    fn get_modifier_type(
        action_states: Vec<&ActionState<WorldInteractionAction>>,
    ) -> Option<DragModifier> {
        for action_state in action_states {
            if action_state.pressed(&WorldInteractionAction::PrimaryDragModifier) {
                return Some(DragModifier::Primary);
            } else if action_state.pressed(&WorldInteractionAction::SecondaryDragModifier) {
                return Some(DragModifier::Secondary);
            }
        }

        None
    }

    let location = drag.hit.position;
    if let Some(location) = location {
        let modifier_type: Option<DragModifier> = get_modifier_type(
            interaction_action_query
                .iter()
                .map(|(state,)| state)
                .collect(),
        );
        println!(
            "Drag started on location: {:?} with modifier: {:?}",
            location, modifier_type
        );

        let event = MapDragStartEvent {
            coordinate: IVec2::new(location.x as i32, location.z as i32),
            drag_modifier: modifier_type,
            selection_type: if drag.button == PointerButton::Primary { SelectionType::Primary } else { SelectionType::Secondary },
        };
        
        map_drag_start_event_writer.send(event);

        drag_info_resource.is_dragging = true;
        drag_info_resource.map_drag_start_event = Some(event);

    }
}

fn handle_ground_plane_drag_end(
    _drag: Trigger<Pointer<DragEnd>>,
    mut map_drag_end_event_writer: EventWriter<MapDragEndEvent>,
    current_mouse_world_coordinate: Res<CurrentMouseWorldCoordinate>,
    mut drag_info_resource: ResMut<DragInfo>,
) {
    let location = current_mouse_world_coordinate.0;
    println!("Drag ended on location: {:?}", location);
    map_drag_end_event_writer.send(MapDragEndEvent(location));
    drag_info_resource.is_dragging = false;
    drag_info_resource.map_drag_start_event = None;
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
        let new_mouse_coordinate = IVec2::new(location.x as i32, location.z as i32);

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

#[derive(Resource, Default, Copy, Clone)]
pub struct DragInfo {
    pub map_drag_start_event: Option<MapDragStartEvent>,
    pub is_dragging: bool,
}

pub fn handle_mouse_dragged(
    drag_info: Res<DragInfo>,
    mut selected_coordinates: ResMut<SelectedCoordinates>,
    current_coordinate: Res<CurrentMouseWorldCoordinate>,
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
                selected_coordinates.0 = rectangle_select(&current_coordinate, event, true);
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
    Bresenham::new(
        (start_coordinate.x as isize, start_coordinate.y as isize),
        (end_coordinate.x as isize, end_coordinate.y as isize),
    )
    .map(|point| IVec2::new(point.0 as i32, point.1 as i32))
    .collect()
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