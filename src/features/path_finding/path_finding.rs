use crate::bundles::settler::Settler;
use crate::features::ai::PathFollow;
use crate::features::map::map_model::MapData;
use crate::features::movement::Velocity;
use crate::features::position::WorldPosition;
use bevy::prelude::*;
use bevy::tasks::futures_lite::future;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use pathfinding::grid::Grid;
use pathfinding::prelude::bfs;
use crate::features::path_finding::grid::PathingGridResource;

#[derive(Debug)]
pub struct Path {
    pub steps: Vec<UVec2>,
    pub related_task: Option<Entity>,
}

#[derive(Component)]
pub struct PathfindingTask(Task<Option<Path>>);

pub fn spawn_pathfinding_task(
    commands: &mut Commands,
    target_entity: Entity,
    grid: &Grid,
    map_data: &MapData,
    start: WorldPosition,
    end: WorldPosition,
    related_task: Option<Entity>,
) {
    //
    let thread_pool = AsyncComputeTaskPool::get();
    let grid = Box::new(grid.clone());
    let start = map_data.world_position_to_top_left_coordinate(start.0);
    let end = map_data.world_position_to_top_left_coordinate(end.0);

    let task = thread_pool.spawn(async move {
        let is_occupied = !grid.has_vertex((end.x as usize, end.y as usize));
        let mut final_end = end;
        if is_occupied {
            let neighbours = grid.neighbours((end.x as usize, end.y as usize));
            let mut found_neighbour = false;
            // TODO: Sort by distance to start
            for neighbour in neighbours {
                if grid.has_vertex((neighbour.0, neighbour.1)) {
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
        let points = bfs(
            &start,
            |p| {
                grid.neighbours((p.x as usize, p.y as usize))
                    .iter()
                    .map(|p| UVec2::new(p.0 as u32, p.1 as u32))
                    .collect::<Vec<_>>()
            },
            |p| *p == final_end,
        );
        println!(
            "from: {:?} to: {:?}, found path: {:?}",
            start,
            end,
            points.is_some()
        );
        //println!("grid: {:?}", grid);
        points.map(|points| Path {
            steps: points,
            related_task,
        })
    });

    println!("Pathfinding task spawned for agent: {:?}", target_entity);
    commands.entity(target_entity).insert(PathfindingTask(task));
}

pub fn apply_pathfinding_result(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut PathfindingTask)>,
) {
    for (task_entity, mut task) in &mut tasks {
        if let Some(result) = future::block_on(future::poll_once(&mut task.0)) {
            commands.entity(task_entity).remove::<PathfindingTask>();

            if let Some(path) = result {
                //println!("Has path! {:?}", path);
                commands.entity(task_entity).insert(PathFollow {
                    path,
                    ..Default::default()
                });
            }
        }
    }
}

pub enum PathFollowResult {
    Success,
    Failure,
}

#[derive(Event)]
pub struct PathFollowFinished {
    pub result: PathFollowResult,
    pub related_task: Option<Entity>,
}

pub fn follow_path(
    mut query: Query<(Entity, &mut PathFollow, &WorldPosition, &mut Velocity)>,
    map_data: Query<&MapData>,
    mut commands: Commands,
) {
    const AT_POINT_THRESHOLD: f32 = 0.001;

    for (entity, mut path_follow, world_position, mut velocity) in query.iter_mut() {
        //println!("{:?}", path_follow);

        if path_follow.finished {
            continue;
        }

        let current_index = path_follow.current_path_index;
        let current_point = path_follow.path.steps[current_index];
        let next_point = path_follow.path.steps[current_index + 1];

        let world_position = map_data
            .get_single()
            .unwrap()
            .world_position_to_top_left_coordinate(world_position.0);

        let direction = (next_point.as_vec2() - world_position.as_vec2()).normalize_or_zero();
        let speed = 3.0;
        let final_vector = Vec2::new(direction.x, direction.y) * speed;
        velocity.0 = final_vector;

        if world_position.as_vec2().distance(next_point.as_vec2()) <= AT_POINT_THRESHOLD {
            if current_index < path_follow.path.steps.len() - 2 {
                path_follow.current_path_index += 1;
            } else {
                path_follow.finished = true;
                commands.entity(entity).trigger(PathFollowFinished {
                    result: PathFollowResult::Success,
                    related_task: path_follow.path.related_task,
                });
            }
        }
    }
}

#[allow(unused, reason="For testing")]
pub fn test_add_pathfinding_task_to_settler(
    added_settler: Query<(Entity, &WorldPosition), Added<Settler>>,
    mut commands: Commands,
    map_data: Query<&MapData>,
    pathing_grid: Res<PathingGridResource>,
) {
    for (entity, world_position) in added_settler.iter() {
        println!("Adding pathfinding to settler");
        let end = WorldPosition(Vec2::new(2.0, 2.0));
        spawn_pathfinding_task(
            &mut commands,
            entity,
            &pathing_grid,
            map_data.single(),
            *world_position,
            end,
            None,
        );
    }
}
