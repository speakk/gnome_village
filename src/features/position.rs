use crate::features::misc_components::InWorld;
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy_spatial::kdtree::KDTree3;
use bevy_spatial::{AutomaticUpdate, SpatialStructure, TransformMode};
use grid_util::Point;
use std::time::Duration;

pub struct PositionPlugin;

impl Plugin for PositionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            AutomaticUpdate::<InWorld>::new()
                .with_spatial_ds(SpatialStructure::KDTree3)
                .with_frequency(Duration::from_secs(1))
                .with_transform(TransformMode::Transform),
        );
    }
}

type SpatialTree = KDTree3<InWorld>;

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut, Reflect)]
#[reflect(Component)]
#[require(PreviousWorldPosition, Transform)]
pub struct WorldPosition(pub Vec2);

impl WorldPosition {
    pub(crate) fn to_point(self) -> Point {
        Point {
            x: self.x as i32,
            y: self.y as i32,
        }
    }
}

/// The value [`WorldPosition`] had in the last fixed timestep.
/// Used for interpolation in the `interpolate_rendered_transform` system.
#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct PreviousWorldPosition(pub Vec2);

// TODO: Code duplication
impl PreviousWorldPosition {
    pub(crate) fn to_point(self) -> Point {
        Point {
            x: self.x as i32,
            y: self.y as i32,
        }
    }
}
