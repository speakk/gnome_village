use crate::bundles::settler::Settler;
use crate::bundles::ResourceItem;
use crate::features::misc_components::InWorld;
use crate::features::position::WorldPosition;
use crate::features::tasks::jobs::Job;
use crate::features::tasks::task::{
    CancelTaskCommand, DestructData, RunType, Task,
    TaskType,
};
use crate::features::world_interaction::destruct_action::DestructTarget;
use bevy::prelude::*;
use std::ops::Mul;

pub fn react_to_destruct_target(
    mut commands: Commands,
    query: Query<(Entity, &WorldPosition), (Added<DestructTarget>, With<InWorld>)>,
) {
    for (entity, world_position) in query.iter() {
        let task_entity = commands
            .spawn((
                Task {
                    run_type: RunType::Leaf,
                    task_type: Some(TaskType::Destruct(DestructData { target: entity })),
                    ..default()
                },
                Job,
                Name::new("Destruct".to_string()),
            ))
            .id();

        commands.entity(entity).observe(
            move |_trigger: Trigger<OnRemove, InWorld>, mut commands: Commands| {
                commands.queue(CancelTaskCommand {
                    reason: "Destruct target removed".to_string(),
                    task_entity,
                });
            },
        );
    }
}

pub fn score_destruct(
    data: &DestructData,
    agents: &[(Entity, &WorldPosition)],
    others_query: &Query<(Entity, &WorldPosition), (Without<ResourceItem>, Without<Settler>)>,
) -> Option<Entity> {
    let target_position = others_query.get(data.target).unwrap().1;

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
