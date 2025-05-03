use crate::bundles::buildables::Buildable;
use crate::features::plants::{GrowthProvider, Plant, PlantLacksGrowthRequirements};
use crate::features::position::{CoordinateToEntity, WorldPosition};
use crate::features::tasks::jobs::{create_bring_resource_task_from_item_amount, Job};
use crate::features::tasks::task::{CancelTaskCommand, RunType, Task};
use bevy::prelude::*;
/*
   Seuraavaksi:
       - Pitää tehdä kasvualusta (joka antaa esim ravinnetta jota kasvi tarvii)
       - Mieti miten nyt maaperässä oleva vesi ei ole saatavilla kasvualustan kasville,
       jotta kasvi tarvitsi myös kastelua
*/

pub fn react_to_lacking_growth_requirements(
    trigger: Trigger<OnAdd, PlantLacksGrowthRequirements>,
    mut commands: Commands,
    query: Query<(&Plant, &WorldPosition)>,
    growth_providers: Query<&GrowthProvider>,
    coordinate_to_entity: Res<CoordinateToEntity>,
) {
    let entity = trigger.target();

    let (plant, world_position) = query.get(entity).unwrap();

    let Some(entities_at_coordinate) = coordinate_to_entity.0.get(&world_position.as_coordinate())
    else {
        return;
    };

    let growth_providers = entities_at_coordinate
        .iter()
        .filter(|entity| growth_providers.contains(**entity))
        .collect::<Vec<_>>();
    let Some(growth_provider) = growth_providers.into_iter().next() else {
        return;
    };

    let task_entity = commands
        .spawn((
            Task {
                run_type: RunType::Parallel,
                ..default()
            },
            Job,
            Name::new("Water plants BringResourcesSequence".to_string()),
        ))
        .with_children(|bring_resource_task| {
            for item_requirement in plant.growth_requirements.as_slice() {
                create_bring_resource_task_from_item_amount(
                    *growth_provider,
                    bring_resource_task,
                    item_requirement,
                );
            }
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
