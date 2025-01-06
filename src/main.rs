mod bundles;
mod features;
mod ui;

use crate::bundles::rock::RockPlugin;
use crate::bundles::BundlePlugin;
use crate::features::camera::CameraPlugin;
use crate::features::map::MapPlugin;
use crate::features::movement::MovementPlugin;
use crate::features::path_finding::PathFindingPlugin;
use crate::features::save::SavePlugin;
use crate::features::sun_light::SunLightPlugin;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MapPlugin)
        .add_plugins(SavePlugin)
        .add_plugins(SunLightPlugin)
        .add_plugins(BundlePlugin)
        .add_plugins(PathFindingPlugin)
        .add_plugins(RockPlugin)
        .add_plugins(features::input::InputPlugin)
        .add_plugins(features::world_interaction::WorldInteractionPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(ui::UiPlugin)
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .run();
}
