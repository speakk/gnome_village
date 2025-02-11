use crate::features::misc_components::InWorld;
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy::utils::{HashMap, HashSet};
use std::time::Duration;

pub struct PositionPlugin;

impl Plugin for PositionPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(CoordinateToEntity::default())
            .add_systems(Update, update_coordinate_to_entity);
    }
}

#[derive(Resource, Debug, Default)]
pub struct CoordinateToEntity(pub(crate) HashMap<IVec2, HashSet<Entity>>);

fn update_coordinate_to_entity(
    mut coordinate_to_entity: ResMut<CoordinateToEntity>,
    query: Query<
        (Entity, &WorldPosition, &PreviousWorldPosition),
        (
            With<InWorld>,
            Or<(Added<WorldPosition>, Changed<WorldPosition>)>,
        ),
    >,
) {
    for (entity, world_position, previous_world_position) in query.iter() {
        let current = world_position.0.as_ivec2();
        let previous = previous_world_position.0.as_ivec2();
        
        
        if let Some(entities_in_previous) = coordinate_to_entity.0.get_mut(&previous) {
            entities_in_previous.retain(|&e| e != entity);
        }
        
        let entities_in_current = coordinate_to_entity.0.entry(current).or_default();
        entities_in_current.insert(entity);
        
        //let mut entities_in_current = coordinate_to_entity.0.get_mut(&current).unwrap_or(&mut vec![]);
        
    }
}

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut, Reflect)]
#[reflect(Component)]
#[require(PreviousWorldPosition, Transform)]
pub struct WorldPosition(pub Vec2);

// impl WorldPosition {
//     pub(crate) fn to_point(self) -> Point {
//         Point {
//             x: self.x as i32,
//             y: self.y as i32,
//         }
//     }
// }

/// The value [`WorldPosition`] had in the last fixed timestep.
/// Used for interpolation in the `interpolate_rendered_transform` system.
#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct PreviousWorldPosition(pub Vec2);

// // TODO: Code duplication
// impl PreviousWorldPosition {
//     pub(crate) fn to_point(self) -> Point {
//         Point {
//             x: self.x as i32,
//             y: self.y as i32,
//         }
//     }
// }
