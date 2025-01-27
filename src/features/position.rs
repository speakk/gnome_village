use bevy::math::Vec2;
use grid_util::Point;
use bevy::prelude::*;

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut, Reflect)]
#[reflect(Component)]
pub struct WorldPosition(pub Vec2);

impl WorldPosition {
    pub(crate) fn to_point(&self) -> Point {
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
    pub(crate) fn to_point(&self) -> Point {
        Point {
            x: self.x as i32,
            y: self.y as i32,
        }
    }
}
