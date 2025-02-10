use crate::bundles::buildables::{BluePrint, BluePrintMaterial, Buildable};
use crate::bundles::{ItemId, ItemSpawners, Prototypes};
use crate::features::map::map_model::MapData;
use crate::features::misc_components::gltf_asset::GltfAsset;
use crate::features::misc_components::simple_mesh::{SimpleMesh, SimpleMeshHandles};
use crate::features::misc_components::{InWorld, Prototype};
use crate::features::position::WorldPosition;
use crate::features::states::AppState;
use crate::features::user_actions::{UserActionIntent, UserActionType};
use crate::features::world_interaction::mouse_selection::{CoordinatesSelectedEvent, CurrentMouseWorldCoordinate, DragInfo, MapClickedEvent, MapDragEndEvent, SelectedCoordinates, SelectionType};
use crate::ui::ui_main_actions::build_menu::BuildMenuBuildableSelected;
use bevy::prelude::*;
use crate::features::world_interaction::mouse_selection;

pub struct BuildActionPlugin;

impl Plugin for BuildActionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentBuilding::default())
            .insert_resource(PreviewEntityHierarchy::default())
            .add_systems(
                Update,
                (
                    react_to_buildable_menu_selected,
                    react_to_build_intent,
                    regenerate_preview_entity,
                    send_build_intent,
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
    drag_info: Res<DragInfo>
) {
    if (!coordinates.is_changed()) && (!current_building.is_changed()) {
        //println!("No changes to coordinates or current building, not regenerating preview entities");
        return;
    }

    if current_building.0.is_none() {
        return;
    }
    
    if drag_info.is_dragging { 
        if let Some(drag_event) = drag_info.map_drag_start_event {
            if drag_event.selection_type != SelectionType::Primary {
                return;
            }
        }
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

fn send_build_intent(
    mut coordinated_selected_events: EventReader<CoordinatesSelectedEvent>,
    mut user_action_intent: EventWriter<UserActionIntent>,
    current_building: Res<CurrentBuilding>,
) {
    let Some(current_building) = current_building.0 else {
        return;
    };

    let Some(event) = coordinated_selected_events.read().last() else {
        return;
    };

    if event.selection_type != SelectionType::Primary {
        return;
    }

    user_action_intent.send(UserActionIntent(UserActionType::Build {
        coordinates: event.coordinates.clone(),
        bundle_type: current_building,
    }));
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

            println!(
                "Got build intent, creating buildables at coordinates: {:?}",
                coordinates
            );
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
