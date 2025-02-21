use crate::features::misc_components::InWorld;
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy::utils::{HashMap, HashSet};

pub struct PositionPlugin;

impl Plugin for PositionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CoordinateToEntity::default())
            .add_systems(Update, update_coordinate_to_entity)
            .add_observer(handle_removed);
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
        let current = world_position.as_coordinate();
        let previous = previous_world_position.as_coordinate();

        if let Some(entities_in_previous) = coordinate_to_entity.0.get_mut(&previous) {
            entities_in_previous.retain(|&e| e != entity);
        }

        let entities_in_current = coordinate_to_entity.0.entry(current).or_default();
        entities_in_current.insert(entity);
    }
}

fn handle_removed(
    trigger: Trigger<OnRemove, WorldPosition>,
    //mut removed_positions: RemovedComponents<WorldPosition>,
    query: Query<(&WorldPosition, &PreviousWorldPosition)>,
    mut coordinate_to_entity: ResMut<CoordinateToEntity>,
) {
    let entity = trigger.entity();
    let Ok((world_position, previous_world_position)) = query.get(entity) else {
        return;
    };

    if let Some(entities) = coordinate_to_entity
        .0
        .get_mut(&previous_world_position.as_coordinate())
    {
        entities.retain(|&e| e != entity)
    }

    if let Some(entities) = coordinate_to_entity.0.get_mut(&world_position.as_coordinate()) {
        entities.retain(|&e| e != entity)
    }
}

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut, Reflect)]
#[reflect(Component)]
#[require(PreviousWorldPosition, Transform)]
pub struct WorldPosition(pub Vec2);

impl WorldPosition {
    pub fn as_coordinate(&self) -> IVec2 {
        self.0.round().as_ivec2()
    }
}

/// The value [`WorldPosition`] had in the last fixed timestep.
/// Used for interpolation in the `interpolate_rendered_transform` system.
#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct PreviousWorldPosition(pub Vec2);

impl PreviousWorldPosition {
    pub fn as_coordinate(&self) -> IVec2 {
        self.0.round().as_ivec2()
    }
}