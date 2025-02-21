use crate::bundles::buildables::BluePrint;
use crate::features::map::map_model::{MapData, TileType};
use crate::features::misc_components::InWorld;
use crate::features::position::{PreviousWorldPosition, WorldPosition};
use crate::ReflectComponent;
use bevy::math::{UVec2, Vec2};
use bevy::prelude::{
    Added, Changed, Component, Deref, DerefMut, IVec2, Query, Reflect, RemovedComponents, ResMut,
    Resource, Single, With, Without,
};
use pathfinding::grid::Grid;
use std::ops::Add;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Solid;

#[derive(Resource, Clone)]
pub struct PathingGridResource(pub Grid);

impl PathingGridResource {
    pub fn convert_to_centered_coordinate(&self, coordinate: UVec2) -> IVec2 {
        let x = (coordinate.x as i32) - (self.0.width as i32) / 2;
        let y = (coordinate.y as i32) - (self.0.height as i32) / 2;
        IVec2::new(x, y)
    }

    pub fn world_position_to_top_left_coordinate(&self, coordinate: Vec2) -> UVec2 {
        let x = coordinate.x + (self.0.width as f32) / 2.0;
        let y = coordinate.y + (self.0.height as f32) / 2.0;
        UVec2::new(x as u32, y as u32)
    }

    // Don't be fooled by the fact that this does nothing, currently coordinates just HAPPEN
    // to match global positions, as tile size is exactly 1,1
    pub fn centered_coordinate_to_world_position(&self, coordinate: IVec2) -> Vec2 {
        let x = coordinate.x as f32;
        let y = coordinate.y as f32;
        Vec2::new(x, y)
    }

    pub fn is_occupied(&self, world_position: &WorldPosition) -> bool {
        let top_left_coordinate = self.world_position_to_top_left_coordinate(world_position.0);
        self.0.has_vertex((
            top_left_coordinate.x as usize,
            top_left_coordinate.y as usize,
        ))
    }

    // Copied from Grid, except modified NOT to return an empty list if the provided
    // vertex is empty in the grid
    pub fn neighbours(
        &self,
        disallow_corner_cutting: bool,
        vertex: (usize, usize),
    ) -> Vec<(usize, usize)> {
        // For now hard code, grid.diagonal_mode is private
        let diagonal_mode = true;
        let (x, y) = vertex;
        let mut candidates = Vec::with_capacity(8);
        if x > 0 {
            candidates.push((x - 1, y));
            if diagonal_mode {
                if y > 0 {
                    candidates.push((x - 1, y - 1));
                }
                if y + 1 < self.0.height {
                    candidates.push((x - 1, y + 1));
                }
            }
        }
        if x + 1 < self.0.width {
            candidates.push((x + 1, y));
            if diagonal_mode {
                if y > 0 {
                    candidates.push((x + 1, y - 1));
                }
                if y + 1 < self.0.height {
                    candidates.push((x + 1, y + 1));
                }
            }
        }
        if y > 0 {
            candidates.push((x, y - 1));
        }
        if y + 1 < self.0.height {
            candidates.push((x, y + 1));
        }
        candidates.retain(|&v| self.0.has_vertex(v));

        if disallow_corner_cutting {
            const CARDINAL_DIRECTIONS: [(isize, isize, [(isize, isize); 2]); 4] = [
                (-1, 0, [(-1, -1), (-1, 1)]), // Left
                (1, 0, [(1, -1), (1, 1)]),    // Right
                (0, -1, [(-1, -1), (1, -1)]), // Up
                (0, 1, [(-1, 1), (1, 1)]),    // Down
            ];

            for &(dx, dy, diagonals) in &CARDINAL_DIRECTIONS {
                if !candidates.contains(&(x.wrapping_add(dx as usize), y.wrapping_add(dy as usize)))
                {
                    for &(ddx, ddy) in &diagonals {
                        if let Some(index) = candidates.iter().position(|&c| {
                            c == (x.wrapping_add(ddx as usize), y.wrapping_add(ddy as usize))
                        }) {
                            candidates.swap_remove(index);
                        }
                    }
                }
            }
        }

        candidates
    }

    // TODO: Sort by distance to point (provide a "target" UVec2 to compare with)
    pub(crate) fn get_nearest_available_vertex(&self, coordinate: IVec2) -> Option<UVec2> {
        let point = self.world_position_to_top_left_coordinate(coordinate.as_vec2());
        let is_end_occupied = !self.0.has_vertex((point.x as usize, point.y as usize));
        let mut final_end = point;

        if is_end_occupied {
            let neighbours = self.neighbours(false, (point.x as usize, point.y as usize));
            let mut found_neighbour = false;
            for neighbour in neighbours {
                if self.0.has_vertex((neighbour.0, neighbour.1)) {
                    final_end = UVec2::new(neighbour.0 as u32, neighbour.1 as u32);
                    found_neighbour = true;
                    break;
                }
            }

            if !found_neighbour {
                // End was occupied, all neighbours are occupied, return None
                return None;
            }
        }

        Some(final_end)
    }

    pub fn get_nearest_available_coordinate(&self, coordinate: IVec2) -> Option<IVec2> {
        let vertex = self.get_nearest_available_vertex(coordinate);
        vertex.map(|vertex| self.convert_to_centered_coordinate(vertex))
    }
}

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
                pathing_grid.0.remove_vertex((x as usize, y as usize));
            } else {
                for world_position in solid_query.iter() {
                    let top_left_coordinate =
                        map_data.world_position_to_top_left_coordinate(world_position.0);
                    pathing_grid.0.remove_vertex((
                        top_left_coordinate.x as usize,
                        top_left_coordinate.y as usize,
                    ));
                }
            }
        }
    }

    println!("{:?}", pathing_grid.0);
}

pub fn react_to_blueprint_removed(
    mut blueprint_removed: RemovedComponents<BluePrint>,
    solid_query: Query<&WorldPosition, (With<Solid>, With<InWorld>)>,
    mut pathing_grid: ResMut<PathingGridResource>,
    map_data: Single<&MapData>,
) {
    for entity in blueprint_removed.read() {
        if let Ok(world_position) = solid_query.get(entity) {
            let top_left_coordinate =
                map_data.world_position_to_top_left_coordinate(world_position.0);

            pathing_grid.0.remove_vertex((
                top_left_coordinate.x as usize,
                top_left_coordinate.y as usize,
            ));
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
