use bevy::asset::uuid::Uuid;
use bevy::color::palettes::basic::GREEN;
use crate::bundles::settler::Settler;
use crate::features::ai::PathFollow;
use crate::features::movement::{MovementIntent};
use crate::features::path_finding::grid::PathingGridResource;
use crate::features::position::WorldPosition;
use bevy::prelude::*;
use bevy::tasks::futures_lite::future;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use pathfinding::prelude::bfs;
use crate::features::tasks::task::TaskFailed;

#[derive(Debug, Clone)]
pub struct Path {
    pub steps: Vec<IVec2>,
    pub pathfinding_id: Uuid,
}

#[derive(Component)]
pub struct PathfindingTask(Task<PathfindingResult>);

pub struct PathfindingResult {
    pub path: Option<Path>,
    pub pathfinding_id: Uuid,
}

pub fn spawn_pathfinding_task(
    commands: &mut Commands,
    target_entity: Entity,
    grid: &PathingGridResource,
    start: WorldPosition,
    end: WorldPosition,
    related_task: Option<Entity>,
) -> Uuid {
    let thread_pool = AsyncComputeTaskPool::get();
    let grid = Box::new(grid.clone());
    let pathfinding_id = Uuid::new_v4();
    
    let task = thread_pool.spawn(async move {
        let start = grid.get_nearest_available_vertex(start.as_coordinate());
        let end = grid.get_nearest_available_vertex(end.as_coordinate());

        if start.is_none() || end.is_none() {
            println!("start or end not found, returning None from Pathfinding task");
            return PathfindingResult {
                path: None,
                pathfinding_id,
            }
        }

        let points = bfs(
            &start.unwrap(),
            |original_point| {
                grid.neighbours(true, (original_point.x as usize, original_point.y as usize))
                    .iter()
                    .map(|p| UVec2::new(p.0 as u32, p.1 as u32))
                    .collect::<Vec<_>>()
            },
            |p| *p == end.unwrap(),
        );

        println!(
            "from: {:?} to: {:?}, found path: {:?}",
            start,
            end,
            points.is_some()
        );
        
        points.map_or(PathfindingResult {
            path: None,
            pathfinding_id,
        }, |points| PathfindingResult {
            path: Some(Path {
                steps: points
                    .iter()
                    .map(|p| grid.convert_to_centered_coordinate(*p))
                    .collect::<Vec<_>>(),
                pathfinding_id
            }), pathfinding_id,
        },)
    });

    println!("Pathfinding task spawned for agent: {:?}", target_entity);
    commands.entity(target_entity).insert(PathfindingTask(task));
    
    pathfinding_id
}

pub fn apply_pathfinding_result(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut PathfindingTask)>,
) {
    for (task_agent, mut task) in &mut tasks {
        if let Some(result) = future::block_on(future::poll_once(&mut task.0)) {
            commands.entity(task_agent).remove::<PathfindingTask>();
            
            if let Some(path) = result.path {
                commands
                    .entity(task_agent)
                    .insert(PathFollow {
                        path,
                        ..Default::default()
                    });
            } else {
                commands.entity(task_agent).trigger(PathFindingFailed { 
                    pathfinding_id: result.pathfinding_id
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
    pub pathfinding_id: Uuid,
}

#[derive(Event)]
pub struct PathFindingFailed {
    pub pathfinding_id: Uuid,
}

pub fn follow_path(
    mut query: Query<(Entity, &mut PathFollow, &WorldPosition, &mut MovementIntent)>,
    mut commands: Commands,
) {
    const AT_POINT_THRESHOLD: f32 = 1.0;

    for (agent_entity, mut path_follow, world_position, mut input) in query.iter_mut() {
        let pathfinding_id = path_follow.path.pathfinding_id;
        
        if path_follow.path.steps.len() == 1 {
            follow_path_succeed(&mut commands, agent_entity, path_follow, &pathfinding_id);
            continue;
        }

        let current_index = path_follow.current_path_index;
        let next_point = path_follow.path.steps[current_index + 1];

        let direction = (next_point.as_vec2() - world_position.0).normalize_or_zero();
        let speed = 4.2;
        let final_vector = Vec2::new(direction.x, direction.y) * speed;
        input.0 = final_vector;

        if world_position.0.distance(next_point.as_vec2()) <= AT_POINT_THRESHOLD {
            if current_index < path_follow.path.steps.len() - 2 {
                path_follow.current_path_index += 1;
            } else {
                follow_path_succeed(&mut commands, agent_entity, path_follow, &pathfinding_id);
            }
        }
    }
}

fn follow_path_succeed(
    commands: &mut Commands,
    agent: Entity,
    path_follow: Mut<PathFollow>,
    pathfinding_id: &Uuid,
) {
    commands
        .entity(agent)
        .trigger(PathFollowFinished {
            result: PathFollowResult::Success,
            pathfinding_id: *pathfinding_id,
        })
        .remove::<PathFollow>();
}

pub fn draw_paths_debug(
    query: Query<(&PathFollow)>,
    mut gizmos: Gizmos
) {
    for path_follow in query {
        for coordinates in path_follow.path.steps.windows(2) {
            let start = coordinates[0];
            let end = coordinates[1];
            gizmos.line(
                Vec3::new(start.x as f32, 0.1, start.y as f32),
                Vec3::new(end.x as f32, 0.1, end.y as f32),
                GREEN,
            );
        }
    }
}

#[allow(unused, reason = "For testing")]
pub fn test_add_pathfinding_task_to_settler(
    added_settler: Query<(Entity, &WorldPosition), Added<Settler>>,
    mut commands: Commands,
    pathing_grid: Res<PathingGridResource>,
) {
    for (entity, world_position) in added_settler.iter() {
        println!("Adding pathfinding to settler");
        let end = WorldPosition(Vec2::new(2.0, 2.0));
        spawn_pathfinding_task(
            &mut commands,
            entity,
            &pathing_grid,
            *world_position,
            end,
            None,
        );
    }
}
