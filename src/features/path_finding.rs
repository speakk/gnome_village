use crate::features::ai::PathFollow;
use crate::features::map::map_model::{MapData, TileType};
use crate::features::misc_components::InWorld;
use crate::features::position::{PreviousWorldPosition, WorldPosition};
use crate::features::states::AppState;
use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy::tasks::futures_lite::future;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use pathfinding::grid::Grid;
use pathfinding::prelude::bfs;
use crate::bundles::buildables::BluePrint;
use crate::bundles::settler::Settler;
use crate::features::movement::{Acceleration, Velocity};

pub struct PathFindingPlugin;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Solid;

#[derive(Resource, Deref, DerefMut)]
pub struct PathingGridResource(pub Grid);

impl Plugin for PathFindingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup)
            // TODO: Add back in, gave weird overflow issue
            .add_systems(
                Update,
                (
                    update_grid_from_solid_component.run_if(in_state(AppState::InGame)),
                    apply_pathfinding_result.run_if(in_state(AppState::InGame)),
                    follow_path.run_if(in_state(AppState::InGame)),
                    //test_add_pathfinding_task_to_settler.run_if(in_state(AppState::InGame)),
                ),
            )
            .insert_resource(PathingGridResource(Grid::new(0, 0)));
    }
}

fn setup(
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
                    pathing_grid.remove_vertex((top_left_coordinate.x as usize, top_left_coordinate.y as usize));
                }
            }
        }
    }
}

#[allow(clippy::type_complexity)]
fn update_grid_from_solid_component(
    mut pathing_grid: ResMut<PathingGridResource>,
    solid_added_query: Query<&WorldPosition, (Added<Solid>, With<InWorld>, Without<BluePrint>)>,
    position_changed_query: Query<
        (&WorldPosition, &PreviousWorldPosition),
        (Changed<WorldPosition>, With<Solid>, With<InWorld>, Without<BluePrint>),
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

        pathing_grid.0.remove_vertex(
            (top_left_coordinate.x as usize, top_left_coordinate.y as usize),
        );

        updated_something = true;
    }

    for (world_position, previous_world_position) in position_changed_query.iter() {
        let top_left_previous =
            map_data.world_position_to_top_left_coordinate(previous_world_position.0);
        let top_left_current = map_data.world_position_to_top_left_coordinate(world_position.0);

        pathing_grid.0.add_vertex(
            (top_left_previous.x as usize, top_left_previous.y as usize),
        );
        pathing_grid.0.remove_vertex(
            (top_left_current.x as usize, top_left_current.y as usize),
        );
        updated_something = true;
    }

    for entity in solid_removed_entities.read() {
        if let Ok(world_position) = world_position_query.get(entity) {
            let top_left_coordinate =
                map_data.world_position_to_top_left_coordinate(world_position.0);
            pathing_grid.0.add_vertex(
                (top_left_coordinate.x as usize, top_left_coordinate.y as usize),
            );
            updated_something = true;
        }
    }

    if updated_something {
        println!("Something updated in map");
        //pathing_grid.generate_components();
    }
}

#[derive(Debug)]
pub struct Path {
    pub steps: Vec<UVec2>,
}

#[derive(Debug)]
pub struct PathfindingError;

#[derive(Component)]
pub struct PathfindingTask(Task<Option<Path>>);

pub fn spawn_pathfinding_task(
    commands: &mut Commands,
    target_entity: Entity,
    grid: &Grid,
    map_data: &MapData,
    start: WorldPosition,
    end: WorldPosition,
) {
    // 
    let thread_pool = AsyncComputeTaskPool::get();
    let grid = Box::new(grid.clone());
    let start = map_data.world_position_to_top_left_coordinate(start.0);
    let end = map_data.world_position_to_top_left_coordinate(end.0);
    
    let task = thread_pool.spawn(async move {
        let is_occupied = !grid.has_vertex((end.x as usize, end.y as usize));
        let mut end = end;
        if is_occupied {
            let neighbours = grid.neighbours((end.x as usize, end.y as usize));
            // TODO: Sort by distance to start
            for neighbour in neighbours {
                if grid.has_vertex((neighbour.0 as usize, neighbour.1 as usize)) {
                    end = UVec2::new(neighbour.0 as u32, neighbour.1 as u32);
                    break;
                }
            }
            
            // End was occupied, all neighbours are occupied, return None
            return None;
        }
        let points = bfs(&start,
                         |p| grid.neighbours((p.x as usize, p.y as usize)).iter().map(|p| UVec2::new(p.0 as u32, p.1 as u32)).collect::<Vec<_>>(),
                         |p| *p == end);
        println!("from: {:?} to: {:?} - {:?}", start, end, points);
        println!("grid: {:?}", grid);
        if let Some(points) = points {
            Some(Path { steps: points })
        } else {
            None
        }
    });
    
    commands.entity(target_entity).insert(PathfindingTask(task));
}

pub fn apply_pathfinding_result(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut PathfindingTask)>,
) {
    for (task_entity, mut task) in &mut tasks {
        println!("Has task...");
        if let Some(result) = future::block_on(future::poll_once(&mut task.0)) {
            commands.entity(task_entity).remove::<PathfindingTask>();
            println!("Has path result!");

            if let Some(path) = result {
                println!("Has path! {:?}", path);
                commands.entity(task_entity).insert(PathFollow {
                    path,
                    ..Default::default()
                });
            }
        }
    }
}

pub fn follow_path(mut query: Query<(&mut PathFollow, &WorldPosition, &mut Velocity)>, map_data: Query<&MapData>) {
    const AT_POINT_THRESHOLD: f32 = 0.001;

    for (mut path_follow, world_position, mut velocity) in query.iter_mut() {
        //println!("{:?}", path_follow);

        if (path_follow.finished) {
            continue;
        }

        let current_index = path_follow.current_path_index;
        let current_point = path_follow.path.steps[current_index];
        let next_point = path_follow.path.steps[current_index + 1];
        
        let world_position = map_data.get_single().unwrap().world_position_to_top_left_coordinate(world_position.0);
        
        let direction = (next_point.as_vec2() - world_position.as_vec2()).normalize_or_zero();
        let speed = 2.0;
        let final_vector = Vec2::new(direction.x as f32, direction.y as f32) * speed;
        velocity.0 = final_vector;

        if (world_position.as_vec2().distance(next_point.as_vec2()) <= AT_POINT_THRESHOLD) {
            if (current_index < path_follow.path.steps.len() - 2) {
                path_follow.current_path_index += 1;
            } else {
                path_follow.finished = true;
            }
        }
    }
}

pub fn test_add_pathfinding_task_to_settler(added_settler: Query<(Entity, &WorldPosition), Added<Settler>>,
                                            mut commands: Commands,
                                            map_data: Query<&MapData>,
                                            pathing_grid: Res<PathingGridResource>,
) {
    for (entity, world_position) in added_settler.iter() {
        println!("Adding pathfinding to settler");
        let end = WorldPosition(Vec2::new(2.0, 2.0));
        spawn_pathfinding_task(&mut commands, entity, &pathing_grid, map_data.single(), *world_position, end);
    }
}