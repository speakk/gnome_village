use crate::bundles::buildables::{BluePrint, Buildable};
use crate::features::misc_components::InWorld;
use crate::features::position::WorldPosition;
use crate::features::tasks::task::{
    BringResourceData, DepositTarget, ItemAmount, RunType, Task, TaskType,
};
use bevy::prelude::*;

pub fn react_to_blueprints(
    mut commands: Commands,
    new_blueprints_query: Query<
        (&BluePrint, &Buildable, &WorldPosition),
        (Added<BluePrint>, With<InWorld>),
    >,
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
                            // TODO: For now just split into 1 task each. In the future do splitting as needed
                            // (depending on carry capacity of worker, etc)
                            for _ in 0..item_requirement.amount {
                                bring_resource_task.spawn((Task {
                                    run_type: RunType::Leaf,
                                    task_type: Some(TaskType::BringResource(BringResourceData {
                                        item_requirement: ItemAmount {
                                            item_id: item_requirement.item_id,
                                            amount: 1,
                                        },
                                        target: DepositTarget::Coordinate(
                                            world_position.0.as_ivec2(),
                                        ),
                                        run_time_data: None,
                                    })),
                                    ..default()
                                },));
                            }
                        }
                    });
            });
    }
}
