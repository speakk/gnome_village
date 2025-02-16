use beet::prelude::*;
use bevy::prelude::*;
use crate::bundles::settler::Settler;
use crate::features::ai::actions::escape_from_solid::EscapeFromSolidAction;
use crate::features::ai::TargetEntity;
use crate::features::misc_components::{InWorld, Prototype};
use crate::features::path_finding::grid::PathingGridResource;
use crate::features::position::WorldPosition;

pub fn attach_to_settler(query: Query<Entity, (Added<Settler>, With<InWorld>, Without<Prototype>)>, mut commands: Commands) {
    for entity in query.iter() {
        let action = commands.spawn((HighestScore::default(),))
            .with_children(|root| {
                root.spawn((
                    StuckInWallScorer,
                    EscapeFromSolidAction,
               ));
            }).id();

        commands.entity(action).trigger(OnRunAction::new(action, entity, ()));
    }
}

#[action(provide_score)]
#[derive(Debug, Clone, PartialEq, Component, Reflect)]
#[reflect(Component)]
struct StuckInWallScorer;

fn provide_score(trigger: Trigger<OnRun<RequestScore>>,
                 mut commands: Commands,
                 query: Query<(
                     &Parent,
                     &TargetEntity,
                 )>,
                 world_positions: Query<&WorldPosition>,
                 pathing_grid: Res<PathingGridResource>
) {
    let target_entity = trigger.origin;
    // let (parent, target_entity) = query
    //     .get(trigger.entity()).expect("Target or parent not found");

    let world_position = world_positions.get(target_entity).expect("WorldPosition not found for target entity");
    
    let score = if pathing_grid.is_occupied(world_position) {
        1.0
    } else {
        0.0
    };

    trigger.trigger_result(&mut commands, ScoreValue(score));
}