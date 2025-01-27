use crate::features::map::map_model::{MapData, TileType};
use crate::features::position::{PreviousWorldPosition, WorldPosition};
use crate::features::states::AppState;
use bevy::app::{App, Plugin};
use bevy::prelude::*;
use grid_pathfinding::PathingGrid;
use grid_util::grid::Grid;
use grid_util::Point;

pub struct PathFindingPlugin;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Solid;

#[derive(Resource, Deref, DerefMut)]
struct PathingGridResource(pub PathingGrid);

impl Plugin for PathFindingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup)
            .add_systems(
                Update,
                update_grid_from_solid_component.run_if(in_state(AppState::InGame)),
            )
            .insert_resource(PathingGridResource(PathingGrid::new(0, 0, false)));
    }
}

fn setup(
    map_data: Query<&MapData, Added<MapData>>,
    mut pathing_grid: ResMut<PathingGridResource>,
    solid_query: Query<&WorldPosition, With<Solid>>,
) {
    if let Ok(map_data) = map_data.get_single() {
        let new_grid = PathingGrid::new(map_data.size.x as usize, map_data.size.y as usize, false);
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
    for x in 0..map_data.size.x {
        for y in 0..map_data.size.y {
            let tile_data = map_data.get_tile_type_non_centered(UVec2::new(x, y));
            let solid = tile_data.is_none_or(|val| val != TileType::Empty);

            if solid {
                pathing_grid.set_point(Point::new(x as i32, y as i32), true);
            } else {
                for world_position in solid_query.iter() {
                    let top_left_coordinate =
                        map_data.world_position_to_top_left_coordinate(world_position.0);
                    pathing_grid.set_point(
                        Point::new(top_left_coordinate.x as i32, top_left_coordinate.y as i32),
                        true,
                    );
                }
            }
        }
    }

    println!("Generate components in do_full_grid_reset");
    pathing_grid.generate_components();
}

#[allow(clippy::type_complexity)]
fn update_grid_from_solid_component(
    mut pathing_grid: ResMut<PathingGridResource>,
    solid_added_query: Query<&WorldPosition, Added<Solid>>,
    position_changed_query: Query<
        (&WorldPosition, &PreviousWorldPosition),
        (Changed<WorldPosition>, With<Solid>),
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

        pathing_grid.0.set_point(
            Point::new(top_left_coordinate.x as i32, top_left_coordinate.y as i32),
            true,
        );

        updated_something = true;
    }

    for (world_position, previous_world_position) in position_changed_query.iter() {
        pathing_grid.set_point(previous_world_position.to_point(), false);
        pathing_grid.set_point(world_position.to_point(), true);
        updated_something = true;
    }

    for entity in solid_removed_entities.read() {
        if let Ok(world_position) = world_position_query.get(entity) {
            pathing_grid.set_point(world_position.to_point(), false);
            updated_something = true;
        }
    }

    if updated_something {
        println!("Generate components in update_grid_from_solid_component");
        pathing_grid.generate_components();
    }
}
