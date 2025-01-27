use crate::bundles::{make_concrete_from_prototype, ItemId, ItemSpawners};
use crate::features::map::map_model::MapData;
use crate::features::misc_components::Prototype;
use crate::features::position::WorldPosition;
use crate::features::user_actions::{UserActionIntent, UserActionState, UserActionType};
use crate::features::world_interaction::mouse_selection::{CurrentMouseWorldCoordinate, MapClickedEvent, MapDragEndEvent, MapDragStartEvent};
use crate::ui::ui_main_actions::build_menu::BuildMenuBuildableSelected;
use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use crate::bundles::buildables::BluePrint;
use crate::features::states::AppState;
use crate::utils::entity_clone::CloneEntityCommandsExt;

pub struct BuildActionPlugin;

impl Plugin for BuildActionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentBuilding::default())
            .insert_resource(CurrentBuildingPreview::default())
            .insert_resource(SelectedCoordinates::default())
            .insert_resource(PreviewEntityHierarchy::default())
            .insert_resource(DragInfo::default())
            .add_systems(
                Update,
                (
                    react_to_buildable_menu_selected,
                    ensure_building_preview,
                    follow_mouse_cursor,
                    //react_to_mouse_clicked,
                    react_to_build_intent,
                    regenerate_preview_entities,
                    react_to_mouse_drag_started,
                    react_to_mouse_drag_ended,
                    handle_mouse_dragged,
                ).run_if(in_state(AppState::InGame)),
            );
    }
}

#[derive(Component, Default)]
pub struct FollowMouseCursor;

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



//fn ensure_building_preview(mut commands: Commands, mut current_building: ResMut<CurrentBuilding>, world: &mut World) {
fn ensure_building_preview(current_building: Res<CurrentBuilding>,
                           mut current_building_preview: ResMut<CurrentBuildingPreview>,
                           mut commands: Commands,
                           item_spawners: Res<ItemSpawners>
) {
    if !current_building.is_changed() {
        return;
    }
    println!("Ensuring building preview");
    if let Some(entity) = current_building_preview.0 {
        if let Some(mut entity_commands) = commands.get_entity(entity) {
            println!("CurrentBuildingPreview Entity exists, despawn");
            entity_commands.despawn();
        }
    }

    println!("Current building was changed");
    if let Some(item_id) = current_building.0 {
        println!("Current building wasn't empty, cloning entity and inserting follow mouse cursor component and removing prototype");
        let new_entity = item_spawners.0.get(&item_id).unwrap()(&mut commands);
        commands
            .entity(new_entity)
            .insert((Visibility::Visible, FollowMouseCursor))
            .remove::<Prototype>();
        println!("Setting current building preview");
        current_building_preview.0 = Some(new_entity);
    }
}

pub fn follow_mouse_cursor(
    mut query: Query<(&mut WorldPosition, Entity), With<FollowMouseCursor>>,
    current_mouse_coordinate: Res<CurrentMouseWorldCoordinate>,
    map_data: Query<&MapData>,
    added_query: Query<Entity, Added<FollowMouseCursor>>,
) {
    for (mut world_position, entity) in query.iter_mut() {
        if current_mouse_coordinate.is_changed() || added_query.contains(entity) {
            let map_data = map_data.single();
            let location =
                map_data.centered_coordinate_to_world_position(current_mouse_coordinate.0);
            *world_position = WorldPosition(location);
        }
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

fn regenerate_preview_entities(
    coordinates: Res<SelectedCoordinates>,
    mut preview_entity_hierarchy: ResMut<PreviewEntityHierarchy>,
    current_building: Res<CurrentBuilding>,
    map_data_query: Query<&MapData>,
    mut commands: Commands,
    item_spawners: Res<ItemSpawners>,
) {
    if (!coordinates.is_changed()) && (!current_building.is_changed()) {
        //println!("No changes to coordinates or current building, not regenerating preview entities");
        return;
    }
    
    //println!("Got through checks, regenerating preview entities indeed");
    
    if let Some(preview_entity_hierarchy) = preview_entity_hierarchy.0 {
        commands.entity(preview_entity_hierarchy).despawn_recursive();
    }
    
    let parent_entity = commands.spawn(Transform::from_xyz(0.0, 0.0, 0.0)).id();
    preview_entity_hierarchy.0 = Some(parent_entity);
    
    let map_data = map_data_query.single();
    
    for coordinate in coordinates.0.iter() {
        println!("Clonin', coordinate: {:?}", coordinate);
        //let cloned = commands.clone_entity(current_building.0.unwrap());
        let cloned = item_spawners.0.get(&current_building.0.unwrap()).unwrap()(&mut commands);
        commands
            .entity(cloned)
            .insert(Transform::from_xyz(coordinate.x as f32, 0.5, coordinate.y as f32, ))
            //.insert(Transform::default())
            .insert(BluePrint)
            .remove::<Prototype>()
            .insert(WorldPosition(map_data.centered_coordinate_to_world_position(*coordinate)))
            .set_parent(parent_entity);
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
                        mut current_coordinate: ResMut<CurrentMouseWorldCoordinate>,
                        mut commands: Commands) {
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
