use moonshine_core::prelude::ReflectMapEntities;
use crate::bundles::buildables::{BluePrint, Buildable};
use crate::bundles::settler::Settler;
use crate::bundles::{Id, Reservations, ResourceItem};
use crate::features::misc_components::InWorld;
use crate::features::position::WorldPosition;
use crate::features::tasks::jobs::{create_bring_resource_task_from_item_amount, Job};
use crate::features::tasks::task::{CancelTaskCommand, ResourceFilter, ResourceQuery, RunType, Task, TaskType};
use bevy::prelude::*;
use std::ops::Mul;
use bevy::ecs::entity::MapEntities;

pub fn react_to_blueprints(
    mut commands: Commands,
    new_blueprints_query: Query<
        (Entity, &BluePrint, &Buildable),
        (Added<BluePrint>, With<InWorld>),
    >,
    tasks: Query<&BuildTask>,
) {
    for (entity, blueprint, buildable) in new_blueprints_query.iter() {
        let task_exists = tasks.iter().any(|task| {
            if task.target == entity {
                return true;
            }
            false
        });

        if task_exists {
            continue;
        }

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
                    BuildTask { target: entity },
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

impl TaskType for BuildTask {
    fn score(
        &mut self,
        mut resources_query: &mut Query<
            ResourceQuery,
            ResourceFilter
        >,
        agents: &[(Entity, &WorldPosition)],
        others_query: &Query<(Entity, &WorldPosition), (Without<ResourceItem>, Without<Settler>)>,
    ) -> Option<Entity> {
        let target_position = others_query.get(self.target).unwrap().1;

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
}

#[derive(Component, Debug, Clone, PartialEq, Eq, Hash, Reflect, MapEntities)]
#[require(Task = Task {
    run_type: RunType::Leaf,
    ..Default::default()
})]
#[reflect(Component, MapEntities)]
pub struct BuildTask {
    #[entities]
    pub target: Entity,
}