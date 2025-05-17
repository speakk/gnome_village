use crate::features::tasks::task::{ResourceFilter, ResourceQuery, Task};
use moonshine_core::prelude::ReflectMapEntities;
use crate::features::tasks::task::RunType;
use crate::bundles::settler::Settler;
use crate::bundles::{Id, Reservations, ResourceItem};
use crate::features::misc_components::destruct_target::DestructTarget;
use crate::features::misc_components::InWorld;
use crate::features::position::WorldPosition;
use crate::features::tasks::jobs::Job;
use crate::features::tasks::task::{CancelTaskCommand, TaskType};
use bevy::prelude::*;
use std::ops::Mul;
use bevy::ecs::entity::MapEntities;

pub fn react_to_destruct_target(
    mut commands: Commands,
    query: Query<(Entity), (Added<DestructTarget>, With<InWorld>)>,
) {
    for (entity) in query.iter() {
        let task_entity = commands
            .spawn((
                DestructTask { target: entity },
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

impl TaskType for DestructTask {
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
pub struct DestructTask {
    #[entities]
    pub target: Entity,
}