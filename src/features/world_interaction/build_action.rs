use crate::bundles::{make_concrete_from_prototype, ItemId, ItemSpawners, Prototypes};
use crate::features::map::map_model::MapData;
use crate::features::misc_components::{GltfAsset, Prototype};
use crate::features::position::WorldPosition;
use crate::features::user_actions::{UserActionIntent, UserActionState, UserActionType};
use crate::features::world_interaction::mouse_selection::{CurrentMouseWorldCoordinate, MapClickedEvent, MapDragEndEvent, MapDragStartEvent};
use crate::ui::ui_main_actions::build_menu::BuildMenuBuildableSelected;
use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use moonshine_object::{Object, ObjectInstance};
use moonshine_view::{BuildView, RegisterView, ViewCommands, Viewable};
use crate::bundles::buildables::{BluePrint, BluePrintMaterial, Buildable};
use crate::features::misc_components::simple_mesh::{SimpleMesh, SimpleMeshHandles, SimpleMeshType};
use crate::features::states::AppState;
use crate::utils::entity_clone::CloneEntityCommandsExt;

pub struct BuildActionPlugin;

impl Plugin for BuildActionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentBuilding::default())
            .insert_resource(CurrentBuildingPreview::default())
            .insert_resource(SelectedCoordinates::default())
            .insert_resource(DragInfo::default())
            .insert_resource(PreviewEntityHierarchy::default())
            //.add_viewable::<PreviewEntityData>()
            .add_systems(
                Update,
                (
                    react_to_buildable_menu_selected,
                    //react_to_mouse_clicked,
                    react_to_build_intent,
                    regenerate_preview_entity,
                    react_to_mouse_drag_started,
                    react_to_mouse_drag_ended,
                    handle_mouse_dragged,
                ).run_if(in_state(AppState::InGame)),
            );
    }
}

#[derive(Resource, Default, Debug, Deref, DerefMut)]
struct CurrentBuilding(Option<ItemId>);

#[derive(Resource, Default, Debug, Deref, DerefMut)]
struct CurrentBuildingPreview(Option<Entity>);

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
    for event in event_reader.read() {
        println!("Reacting to mouse clicked, sending build intent");
        if let Some(current_building) = current_building.0 {
            event_writer.send(UserActionIntent(UserActionType::Build {
                bundle_type: current_building,
                coordinate: coordinate.0,
            }));
        }
    }
}

#[derive(Resource, Default, Copy, Clone)]
struct DragInfo {
    map_drag_start_event: Option<MapDragStartEvent>,
    is_dragging: bool,
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

    println!("Got through checks, regenerating preview entities indeed");

    if let Some(preview_entity_hierarchy) = preview_entity_hierarchy.0 {
        commands.entity(preview_entity_hierarchy).despawn_recursive();
    }
    
    let parent_entity = commands.spawn((Transform::default(), Visibility::Visible)).id();
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
        spawned.insert(Transform::from_xyz(position.x, 0.5, position.y))
            .insert(BluePrint)
            .remove::<Prototype>()
            .set_parent(parent_entity);
        
        if let Some(simple_mesh_data) = simple_mesh_data {
            let mesh_handle = simple_mesh_handles.0.get(&simple_mesh_data.0.clone()).unwrap();
            spawned.insert(Mesh3d(mesh_handle.clone()))
                .insert(MeshMaterial3d(blueprint_material.0.clone().unwrap()));
        }
        
        if let Some(gltf_asset_data) = gltf_asset_data {
            let scene_root = SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(gltf_asset_data.0.clone())));
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
) {
    if let Some(event) = event_reader.read().next() {
        drag_info_resource.is_dragging = false;
        drag_info_resource.map_drag_start_event = None;
        selected_coordinates.0 = Vec::new();
        println!("Reacting to mouse drag ended");
    }
}

fn handle_mouse_dragged(drag_info: Res<DragInfo>,
                        mut selected_coordinates: ResMut<SelectedCoordinates>,
                        current_coordinate: Res<CurrentMouseWorldCoordinate>) {
    if !current_coordinate.is_changed() { return; }

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
fn react_to_build_intent(item_spawners: Res<ItemSpawners>,
                         mut commands: Commands,
                        map_data: Query<&MapData>,
                         mut user_action_intent: EventReader<UserActionIntent>,
) {
    for event in user_action_intent.read() {
        if let UserActionType::Build { bundle_type: item_id, coordinate } = event.0 {
            let concrete_entity = item_spawners.0.get(&item_id).unwrap()(&mut commands);
            let map_data = map_data.get_single().unwrap();

            let world_position = map_data.centered_coordinate_to_world_position(coordinate);
            commands
                .entity(concrete_entity)
                .insert(WorldPosition(world_position))
                .insert(BluePrint);

            break;
        }
    }
}
