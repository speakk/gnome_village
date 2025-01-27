use crate::bundles::buildables::{BluePrint, Buildable};
use crate::features::misc_components::InWorld;
use crate::features::tasks::task::{BringResourceData, DepositTarget, RunType, Task, TaskType};
use bevy::prelude::*;
use crate::features::position::WorldPosition;

pub fn react_to_blueprints(
    mut commands: Commands,
    new_blueprints_query: Query<(&BluePrint, &Buildable, &WorldPosition), (Added<BluePrint>, With<InWorld>)>,
) {
    for (blueprint, buildable, world_position) in new_blueprints_query.iter() {
        println!("Got blueprint: {:?}", blueprint);
        let new_task = commands
            .spawn((Task {
                run_type: RunType::Sequence,
                ..default()
            },))
            .with_children(|parent_task| {
                let bring_resources = parent_task
                    .spawn((Task {
                        run_type: RunType::Parallel,
                        ..default()
                    },))
                    .with_children(|bring_resource_task| {
                        
                        for item_requirement in buildable.item_requirements.as_slice() {
                            bring_resource_task.spawn((Task {
                                run_type: RunType::Leaf,
                                task_type: Some(TaskType::BringResource(BringResourceData {
                                    item_requirement: item_requirement.clone(),
                                    target: DepositTarget::Coordinate(world_position.0.as_ivec2())
                                })),
                                ..default()
                            },));
                        }
                    });
            });
    }
}
