use crate::features::input::WorldInteractionAction;
use crate::features::map::map_model::MapData;
use crate::features::states::AppState;
use bevy::prelude::KeyCode::{ControlLeft, KeyA, KeyD, ShiftLeft};
use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;
use leafwing_input_manager::input_map::InputMap;
use leafwing_input_manager::InputManagerBundle;

#[derive(Event)]
pub struct MapClickedEvent(pub IVec2);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DragModifier {
    Primary,
    Secondary,
}

#[derive(Event, Clone, Copy)]
pub struct MapDragStartEvent {
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
            .add_event::<MapClickedEvent>()
            .add_event::<MapDragStartEvent>()
            .add_event::<MapDragEndEvent>()
            .add_systems(OnEnter(AppState::InGame), setup)
            .add_systems(
                Update,
                scale_ground_mesh_based_on_map.run_if(in_state(AppState::InGame)),
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
) {
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

        map_drag_start_event_writer.send(MapDragStartEvent {
            coordinate: IVec2::new(location.x as i32, location.z as i32),
            drag_modifier: modifier_type,
        });
    }
}

fn handle_ground_plane_drag_end(
    _drag: Trigger<Pointer<DragEnd>>,
    mut map_drag_end_event_writer: EventWriter<MapDragEndEvent>,
    current_mouse_world_coordinate: Res<CurrentMouseWorldCoordinate>,
) {
    let location = current_mouse_world_coordinate.0;
    println!("Drag ended on location: {:?}", location);
    map_drag_end_event_writer.send(MapDragEndEvent(location));
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
