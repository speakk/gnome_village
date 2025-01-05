use bevy::prelude::*;
use bevy::prelude::KeyCode::{F5, F8};
use leafwing_input_manager::action_state::ActionState;
use leafwing_input_manager::input_map::InputMap;
use leafwing_input_manager::InputManagerBundle;
use crate::features::input::{SaveLoadAction, WorldInteractionAction};
use crate::features::map::map_model::MapData;

#[derive(Event)]
pub struct MapClickedEvent(pub IVec2);

pub struct MouseSelectionPlugin;

#[derive(Component)]
struct PickingGroundPlane;

impl Plugin for MouseSelectionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(MeshPickingPlugin)
            .add_event::<MapClickedEvent>()
            .add_systems(Startup, setup)
            .add_systems(Update, scale_ground_mesh_based_on_map);
    }
}

fn setup(mut commands: Commands, mut mesh_picking_settings: ResMut<MeshPickingSettings>,
         mut meshes: ResMut<Assets<Mesh>>,
         mut materials: ResMut<Assets<StandardMaterial>>,
) {
    println!("Setting up mouse selection plugin");
    mesh_picking_settings.require_markers = true;
    mesh_picking_settings.ray_cast_visibility = RayCastVisibility::Any;
    
    let ground_plane_mesh = Mesh3d(meshes.add(Plane3d::default().mesh().size(200.0, 200.0).subdivisions(10)));
    
    commands.spawn((
        PickingGroundPlane,
        ground_plane_mesh,
        Visibility::Hidden,
        MeshMaterial3d(materials.add(Color::rgb(0.5, 0.0, 0.0))),
        Transform::from_xyz(-0.5, 0.0, -0.5),
        RayCastPickable,
                       )).observe(handle_ground_plane_interaction);
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
        let new_handle = meshes.add(Plane3d::default().mesh().size(map_size.x as f32, map_size.y as f32).subdivisions(10));
        
        mesh_3d.0 = new_handle;
    }
}

fn handle_ground_plane_interaction(click: Trigger<Pointer<Click>>, mut map_clicked_event_writer: EventWriter<MapClickedEvent>) {
    let location = click.hit.position;
    if let Some(location) = location {
        map_clicked_event_writer.send(MapClickedEvent(IVec2::new(location.x as i32, location.z as i32)));        
    }
}