use crate::bundles::buildables::{BluePrint, Buildable};
use crate::bundles::settler::Settler;
use crate::bundles::ResourceItem;
use crate::features::misc_components::InWorld;
use crate::features::position::WorldPosition;
use crate::features::tasks::jobs::{create_bring_resource_task_from_item_amount, Job};
use crate::features::tasks::task::{
    BuildData, CancelTaskCommand, RunType, Task, TaskType,
};
use bevy::prelude::*;
use std::ops::Mul;

pub fn react_to_blueprints(
    mut commands: Commands,
    new_blueprints_query: Query<
        (Entity, &BluePrint, &Buildable),
        (Added<BluePrint>, With<InWorld>),
    >,
) {
    for (entity, blueprint, buildable) in new_blueprints_query.iter() {
        println!("Got blueprint: {:?}", blueprint);
        let task_entity = commands
            .spawn((
                Task {
                    run_type: RunType::Sequence,
                    ..default()
                },
                Job,
                Name::new("BuildTaskTree".to_string()),
            ))
            .with_children(|parent_task| {
                parent_task
                    .spawn((
                        Task {
                            run_type: RunType::Parallel,
                            ..default()
                        },
                        Name::new("BringResourcesSequence".to_string()),
                    ))
                    .with_children(|bring_resource_task| {
                        for item_requirement in buildable.item_requirements.as_slice() {
                            // TODO: For now just split into 1 task each. In the future do splitting as needed
                            // (depending on carry capacity of worker, etc)
                            create_bring_resource_task_from_item_amount(
                                entity,
                                bring_resource_task,
                                item_requirement,
                            );
                        }
                    });

                parent_task.spawn((
                    Name::new("BuildTask".to_string()),
                    Task {
                        run_type: RunType::Leaf,
                        task_type: Some(TaskType::Build(BuildData { target: entity })),
                        ..Default::default()
                    },
                ));
            })
            .id();

        commands.entity(entity).observe(
            move |_trigger: Trigger<OnRemove, Buildable>, mut commands: Commands| {
                commands.queue(CancelTaskCommand {
                    reason: "Target Buildable removed".to_string(),
                    task_entity,
                });
            },
        );
    }
}

pub fn score_build(
    build_data: &BuildData,
    agents: &[(Entity, &WorldPosition)],
    others_query: &Query<(Entity, &WorldPosition), (Without<ResourceItem>, Without<Settler>)>,
) -> Option<Entity> {
    let target_position = others_query.get(build_data.target).unwrap().1;

    let mut best_score = -999999.0;
    let mut best_agent: Option<Entity> = None;

    for (agent, world_position) in agents.iter() {
        let score = target_position.0.distance(world_position.0).mul(-1.0);
        if score > best_score {
            best_score = score;
            best_agent = Some(*agent);
        }
    }

    best_agent
}
