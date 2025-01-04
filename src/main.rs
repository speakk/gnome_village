mod bundles;
mod features;

use crate::bundles::BundlePlugin;
use crate::features::camera::CameraPlugin;
use crate::features::map::MapPlugin;
use crate::features::movement::MovementPlugin;
use crate::features::sun_light::SunLightPlugin;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MapPlugin)
        .add_plugins(SunLightPlugin)
        .add_plugins(BundlePlugin)
        .add_plugins(features::input::InputPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(MovementPlugin)
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .run();
}
