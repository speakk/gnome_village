use crate::bundles::buildables::{BluePrint, BluePrintMaterial, Buildable};
use crate::bundles::{ItemId, ItemSpawners, Prototypes};
use crate::features::map::map_model::MapData;
use crate::features::misc_components::gltf_asset::GltfAsset;
use crate::features::misc_components::simple_mesh::{SimpleMesh, SimpleMeshHandles};
use crate::features::misc_components::{InWorld, Prototype};
use crate::features::position::WorldPosition;
use crate::features::states::AppState;
use crate::features::user_actions::{UserActionIntent, UserActionType};
use crate::features::world_interaction::mouse_selection::{
    CurrentMouseWorldCoordinate, MapClickedEvent, MapDragEndEvent, MapDragStartEvent,
};
use crate::ui::ui_main_actions::build_menu::BuildMenuBuildableSelected;
use bevy::prelude::*;

pub struct BuildActionPlugin;

impl Plugin for BuildActionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentBuilding::default())
            .insert_resource(SelectedCoordinates::default())
            .insert_resource(DragInfo::default())
            .insert_resource(PreviewEntityHierarchy::default())
            .add_systems(
                Update,
                (
                    react_to_buildable_menu_selected,
                    react_to_build_intent,
                    regenerate_preview_entity,
                    react_to_mouse_clicked,
                    react_to_mouse_drag_started,
                    react_to_mouse_drag_ended,
                    handle_mouse_dragged,
                )
                    .run_if(in_state(AppState::InGame)),
            );
    }
}

#[derive(Resource, Default, Debug, Deref, DerefMut)]
struct CurrentBuilding(Option<ItemId>);

fn react_to_buildable_menu_selected(
    mut build_menu_buildable_selected: EventReader<BuildMenuBuildableSelected>,
    mut current_building: ResMut<CurrentBuilding>,
) {
    for event in build_menu_buildable_selected.read() {
        println!("Reacting to buildable menu selected, setting current_building");
        current_building.0 = Some(event.0);
    }
}

fn react_to_mouse_clicked(
    mut event_reader: EventReader<MapClickedEvent>,
    mut event_writer: EventWriter<UserActionIntent>,
    coordinate: Res<CurrentMouseWorldCoordinate>,
    current_building: Res<CurrentBuilding>,
) {
    for _event in event_reader.read() {
        if let Some(current_building) = current_building.0 {
            event_writer.send(UserActionIntent(UserActionType::Build {
                bundle_type: current_building,
                coordinates: vec![coordinate.0],
            }));
        }
    }
}

#[derive(Resource, Default, Copy, Clone)]
pub struct DragInfo {
    pub map_drag_start_event: Option<MapDragStartEvent>,
    pub is_dragging: bool,
}

#[derive(Resource, Default, Deref, DerefMut, Clone)]
struct SelectedCoordinates(Vec<IVec2>);

#[derive(Resource, Default, Deref, DerefMut, Clone)]
struct PreviewEntityHierarchy(Option<Entity>);

#[allow(clippy::too_many_arguments)]
fn regenerate_preview_entity(
    coordinates: Res<SelectedCoordinates>,
    mut preview_entity_hierarchy: ResMut<PreviewEntityHierarchy>,
    current_building: Res<CurrentBuilding>,
    map_data_query: Query<&MapData>,
    simple_mesh_handles: Res<SimpleMeshHandles>,
    mut commands: Commands,
    blueprint_material: Res<BluePrintMaterial>,
    prototypes: Res<Prototypes>,
    render_info_query: Query<(Option<&SimpleMesh>, Option<&GltfAsset>), With<Buildable>>,
    asset_server: Res<AssetServer>,
) {
    if (!coordinates.is_changed()) && (!current_building.is_changed()) {
        //println!("No changes to coordinates or current building, not regenerating preview entities");
        return;
    }

    if current_building.0.is_none() {
        return;
    }

    //println!("Got through checks, regenerating preview entities indeed");

    if let Some(preview_entity_hierarchy) = preview_entity_hierarchy.0 {
        commands
            .entity(preview_entity_hierarchy)
            .despawn_recursive();
    }

    let parent_entity = commands
        .spawn((Transform::default(), Visibility::Visible))
        .id();
    preview_entity_hierarchy.0 = Some(parent_entity);

    let prototype = prototypes.0.get(&current_building.0.unwrap()).unwrap();
    let mut simple_mesh_data: Option<&SimpleMesh> = None;
    let mut gltf_asset_data: Option<&GltfAsset> = None;

    if let Ok((simple_mesh, gltf_asset)) = render_info_query.get(*prototype) {
        simple_mesh_data = simple_mesh;
        gltf_asset_data = gltf_asset;
    }

    let map_data = map_data_query.single();

    for coordinate in coordinates.0.iter() {
        let position = map_data.centered_coordinate_to_world_position(*coordinate);
        let mut spawned = commands.spawn_empty();
        spawned
            .insert(Transform::from_xyz(position.x, 0.5, position.y))
            .insert(BluePrint)
            .remove::<Prototype>()
            .set_parent(parent_entity);

        if let Some(simple_mesh_data) = simple_mesh_data {
            let mesh_handle = simple_mesh_handles
                .0
                .get(&simple_mesh_data.0.clone())
                .unwrap();
            spawned
                .insert(Mesh3d(mesh_handle.clone()))
                .insert(MeshMaterial3d(blueprint_material.0.clone().unwrap()));
        }

        if let Some(gltf_asset_data) = gltf_asset_data {
            let scene_root = SceneRoot(
                asset_server.load(GltfAssetLabel::Scene(0).from_asset(gltf_asset_data.0.clone())),
            );
            spawned.insert(scene_root);
        }
    }
}

fn react_to_mouse_drag_started(
    mut event_reader: EventReader<MapDragStartEvent>,
    mut drag_info_resource: ResMut<DragInfo>,
) {
    if let Some(event) = event_reader.read().next() {
        drag_info_resource.is_dragging = true;
        drag_info_resource.map_drag_start_event = Some(*event);
        println!("Reacting to mouse drag started");
    }
}

fn react_to_mouse_drag_ended(
    mut event_reader: EventReader<MapDragEndEvent>,
    mut drag_info_resource: ResMut<DragInfo>,
    mut selected_coordinates: ResMut<SelectedCoordinates>,
    mut user_action_intent: EventWriter<UserActionIntent>,
    current_building: Res<CurrentBuilding>,
) {
    if let Some(_) = event_reader.read().next() {
        drag_info_resource.is_dragging = false;
        drag_info_resource.map_drag_start_event = None;

        println!("Got mouse drag end event, sending build intent with coordinates: {:?}", selected_coordinates.0);
        
        if current_building.0.is_none() {
            return;
        } else {
            user_action_intent.send(UserActionIntent(UserActionType::Build {
                coordinates: selected_coordinates.0.clone(),
                bundle_type: current_building.0.unwrap(),
            }));
        }

        selected_coordinates.0 = Vec::new();
    }
}

fn handle_mouse_dragged(
    drag_info: Res<DragInfo>,
    mut selected_coordinates: ResMut<SelectedCoordinates>,
    current_coordinate: Res<CurrentMouseWorldCoordinate>,
) {
    if !current_coordinate.is_changed() {
        return;
    }

    if (drag_info.is_dragging) && (drag_info.map_drag_start_event.is_some()) {
        let event = drag_info.map_drag_start_event.unwrap();
        let min_x = current_coordinate.0.x.min(event.coordinate.x);
        let min_y = current_coordinate.0.y.min(event.coordinate.y);
        let max_x = current_coordinate.0.x.max(event.coordinate.x);
        let max_y = current_coordinate.0.y.max(event.coordinate.y);
        let mut new_coordinates = Vec::new();
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                new_coordinates.push(IVec2::new(x, y));
            }
        }
        selected_coordinates.0 = new_coordinates;
    } else {
        selected_coordinates.0 = vec![current_coordinate.0];
    }
}

// TODO: Just validate in this and then emit BuildAction
fn react_to_build_intent(
    item_spawners: Res<ItemSpawners>,
    mut commands: Commands,
    map_data: Query<&MapData>,
    mut user_action_intent: EventReader<UserActionIntent>,
) {
    for event in user_action_intent.read() {
        if let UserActionType::Build {
            bundle_type: item_id,
            coordinates,
        } = event.0.clone()
        {
            let map_data = map_data.get_single().unwrap();

            
            println!("Got build intent, creating buildables at coordinates: {:?}", coordinates);
            for coordinate in coordinates.iter() {
                let concrete_entity = item_spawners.0.get(&item_id).unwrap()(&mut commands);
                let world_position = map_data.centered_coordinate_to_world_position(*coordinate);
                println!("Creating buildable at: {:?}", world_position);
                commands
                    .entity(concrete_entity)
                    .insert(WorldPosition(world_position))
                    .insert(BluePrint)
                    .insert(InWorld);
            }

            break;
        }
    }
}
