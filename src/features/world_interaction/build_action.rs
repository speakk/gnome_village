use crate::bundles::{clone_entity, make_concrete_from_prototype};
use crate::features::map::map_model::MapData;
use crate::features::misc_components::Prototype;
use crate::features::position::WorldPosition;
use crate::features::user_actions::{UserActionIntent, UserActionState, UserActionType};
use crate::features::world_interaction::mouse_selection::{
    CurrentMouseWorldCoordinate, MapClickedEvent,
};
use crate::ui::ui_main_actions::build_menu::BuildMenuBuildableSelected;
use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use crate::bundles::buildables::BluePrint;

pub struct BuildActionPlugin;

impl Plugin for BuildActionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentBuilding::default())
            .insert_resource(CurrentBuildingPreview::default())
            .add_systems(
                Update,
                follow_mouse_cursor.run_if(in_state(UserActionState::PlacingBuilding)),
            )
            .add_systems(
                Update,
                (
                    react_to_buildable_menu_selected,
                    ensure_building_preview,
                    react_to_mouse_clicked,
                    react_to_build_intent,
                ),
            );
    }
}

#[derive(Component, Default)]
pub struct FollowMouseCursor;

#[derive(Resource, Default, Debug, Deref, DerefMut)]
struct CurrentBuilding(Option<Entity>);

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
fn ensure_building_preview(world: &mut World) {
    if !world.is_resource_changed::<CurrentBuilding>() {
        return;
    }
    println!("Ensuring building preview");
    world.resource_scope(
        |world, current_building_preview: Mut<CurrentBuildingPreview>| {
            if let Some(entity) = current_building_preview.0 {
                let mut commands = world.commands();
                if let Some(mut entity_commands) = commands.get_entity(entity) {
                    println!("CurrentBuildingPreview Entity exists, despawn");
                    entity_commands.despawn();
                }
            }
        },
    );

    let current_building = world.resource_mut::<CurrentBuilding>();
    if current_building.is_changed() {
        println!("Current building was changed");
        if let Some(entity) = current_building.0 {
            println!("Current building wasn't empty, cloning entity and inserting follow mouse cursor component and removing prototype");
            let new_entity = clone_entity(world, entity);

            let mut commands = world.commands();
            commands
                .entity(new_entity)
                .insert((Visibility::Visible, FollowMouseCursor))
                .remove::<Prototype>();
            world.resource_scope(
                |world, mut current_building_preview: Mut<CurrentBuildingPreview>| {
                    println!("Setting current building preview");
                    current_building_preview.0 = Some(new_entity);
                },
            );
        }
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
        if let Some(current_building) = current_building.0 {
            event_writer.send(UserActionIntent(UserActionType::Build {
                entity: current_building,
                coordinate: coordinate.0,
            }));
        }
    }
}

fn react_to_build_intent(world: &mut World) {
    let mut event_system_state = SystemState::<EventReader<UserActionIntent>>::new(world);
    let mut events = event_system_state.get_mut(world);

    for event in events.read() {
        if let UserActionType::Build { entity, coordinate } = event.0 {
            let concrete_entity = make_concrete_from_prototype(entity, world);
            let map_data = {
                let mut query = world.query::<&MapData>();
                query.get_single(world).unwrap()
            };
            let world_position = map_data.centered_coordinate_to_world_position(coordinate);
            let mut commands = world.commands();
            commands
                .entity(concrete_entity)
                .insert(WorldPosition(world_position))
                .insert(BluePrint);

            break;
        }
    }
}
