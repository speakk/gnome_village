use crate::ReflectComponent;
use bevy::prelude::{Added, Changed, Component, Deref, DerefMut, Query, Reflect, RemovedComponents, ResMut, Resource, With, Without};
use pathfinding::grid::Grid;
use bevy::math::UVec2;
use crate::bundles::buildables::BluePrint;
use crate::features::map::map_model::{MapData, TileType};
use crate::features::misc_components::InWorld;
use crate::features::position::{PreviousWorldPosition, WorldPosition};

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Solid;

#[derive(Resource, Deref, DerefMut)]
pub struct PathingGridResource(pub Grid);

pub fn setup(
    map_data: Query<&MapData, Added<MapData>>,
    mut pathing_grid: ResMut<PathingGridResource>,
    solid_query: Query<&WorldPosition, With<Solid>>,
) {
    if let Ok(map_data) = map_data.get_single() {
        let mut new_grid = Grid::new(map_data.size.x as usize, map_data.size.y as usize);
        new_grid.enable_diagonal_mode();
        pathing_grid.0 = new_grid;
        do_full_grid_reset(map_data, pathing_grid, solid_query);
        println!("Grid setup");
    }
}

fn do_full_grid_reset(
    map_data: &MapData,
    mut pathing_grid: ResMut<PathingGridResource>,
    solid_query: Query<&WorldPosition, With<Solid>>,
) {
    pathing_grid.0.fill();

    for x in 0..map_data.size.x {
        for y in 0..map_data.size.y {
            let tile_data = map_data.get_tile_type_non_centered(UVec2::new(x, y));
            let solid = tile_data.is_none_or(|val| val != TileType::Dirt);

            if solid {
                pathing_grid.remove_vertex((x as usize, y as usize));
            } else {
                for world_position in solid_query.iter() {
                    let top_left_coordinate =
                        map_data.world_position_to_top_left_coordinate(world_position.0);
                    pathing_grid.remove_vertex((
                        top_left_coordinate.x as usize,
                        top_left_coordinate.y as usize,
                    ));
                }
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn update_grid_from_solid_component(
    mut pathing_grid: ResMut<PathingGridResource>,
    solid_added_query: Query<&WorldPosition, (Added<Solid>, With<InWorld>, Without<BluePrint>)>,
    position_changed_query: Query<
        (&WorldPosition, &PreviousWorldPosition),
        (
            Changed<WorldPosition>,
            With<Solid>,
            With<InWorld>,
            Without<BluePrint>,
        ),
    >,
    mut solid_removed_entities: RemovedComponents<Solid>,
    world_position_query: Query<&WorldPosition>,
    map_data: Query<&MapData>,
) {
    let Ok(map_data) = map_data.get_single() else {
        return;
    };

    let mut updated_something = false;

    for world_position in solid_added_query.iter() {
        let top_left_coordinate = map_data.world_position_to_top_left_coordinate(world_position.0);

        pathing_grid.0.remove_vertex((
            top_left_coordinate.x as usize,
            top_left_coordinate.y as usize,
        ));

        updated_something = true;
    }

    for (world_position, previous_world_position) in position_changed_query.iter() {
        let top_left_previous =
            map_data.world_position_to_top_left_coordinate(previous_world_position.0);
        let top_left_current = map_data.world_position_to_top_left_coordinate(world_position.0);

        pathing_grid
            .0
            .add_vertex((top_left_previous.x as usize, top_left_previous.y as usize));
        pathing_grid
            .0
            .remove_vertex((top_left_current.x as usize, top_left_current.y as usize));
        updated_something = true;
    }

    for entity in solid_removed_entities.read() {
        if let Ok(world_position) = world_position_query.get(entity) {
            let top_left_coordinate =
                map_data.world_position_to_top_left_coordinate(world_position.0);
            pathing_grid.0.add_vertex((
                top_left_coordinate.x as usize,
                top_left_coordinate.y as usize,
            ));
            updated_something = true;
        }
    }

    if updated_something {
        println!("Something updated in map");
        //pathing_grid.generate_components();
    }
}