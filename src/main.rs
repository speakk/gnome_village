mod features;

use crate::features::camera::CameraPlugin;
use crate::features::map::MapPlugin;
use crate::features::movement::MovementPlugin;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MapPlugin)
        .add_plugins(features::input::InputPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(MovementPlugin)
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .run();
}
