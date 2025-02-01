use beet::prelude::{Action, ScoreFlow};
use bevy::prelude::*;
use crate::bundles::settler::Settler;
use crate::features::ai::actions::escape_from_solid::EscapeFromSolidAction;
use crate::features::misc_components::{InWorld, Prototype};
use crate::features::path_finding::grid::PathingGridResource;
use crate::features::position::WorldPosition;

pub fn attach_to_settler(query: Query<Entity, (Added<Settler>, With<InWorld>, Without<Prototype>)>, mut commands: Commands) {
    for entity in query.iter() {
        commands.spawn((ScoreFlow::default(),))
            .with_children(|root| {
                root.spawn((
                    StuckInWallScorer,
                   // SequenceFlow,
                    EscapeFromSolidAction,
                    TargetEntity(entity)
                    // )).with_children(|sequence| {
                    //      sequence.spawn(
                    //          
                    //      )
                    //});
               ));
            });
    }
}

#[derive(Debug, Clone, PartialEq, Component, Reflect, Action)]
#[observers(provide_score)]
#[reflect(Component)]
struct StuckInWallScorer;

fn provide_score(trigger: Trigger<RequestScore>,
                 mut commands: Commands,
                 query: Query<(
                     &Parent,
                     &TargetEntity,
                 )>,
                 world_positions: Query<&WorldPosition>,
                 pathing_grid: Res<PathingGridResource>
) {
    let (parent, target_entity) = query
        .get(trigger.entity()).expect("Target or parent not found");

    let world_position = world_positions.get(target_entity.0).expect("WorldPosition not found for target entity");
    
    let score = if pathing_grid.is_occupied(world_position) {
        1.0
    } else {
        0.0
    };
    
    commands
        .entity(parent.get())
        .trigger(OnChildScore::new(trigger.entity(), score));
}