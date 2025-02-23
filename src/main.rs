mod bundles;
mod features;
mod ui;

use crate::bundles::rock::RockPlugin;
use crate::bundles::BundlePlugin;
use crate::features::ai::AiPlugin;
use crate::features::assets::AssetsPlugin;
use crate::features::camera::CameraPlugin;
use crate::features::inventory::InventoryPlugin;
use crate::features::map::map_model::WorldSeed;
use crate::features::map::MapPlugin;
use crate::features::misc_components::MiscComponentsPlugin;
use crate::features::movement::MovementPlugin;
use crate::features::plants::PlantsPlugin;
use crate::features::position::PositionPlugin;
use crate::features::save::SavePlugin;
use crate::features::states::preload::PreloadPlugin;
use crate::features::states::AppState;
use crate::features::sun_light::SunLightPlugin;
use crate::features::user_actions::UserActionsPlugin;
use beet::prelude::BeetFlowPlugin;
use bevy::input::common_conditions::input_toggle_active;
use bevy::pbr::{DefaultOpaqueRendererMethod, PointLightShadowMap};
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use features::path_finding::plugin::PathFindingPlugin;
use features::tasks::tasks_plugin::TasksPlugin;
use crate::features::item_drop::ItemDropPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BeetFlowPlugin::default())
        .insert_resource(DefaultOpaqueRendererMethod::deferred())
        .insert_resource(PointLightShadowMap { size: 256 })
        .add_plugins(PreloadPlugin)
        .add_plugins(AssetsPlugin)
        .add_plugins(BundlePlugin)
        .add_plugins(MapPlugin)
        .add_plugins(SavePlugin)
        .add_plugins(SunLightPlugin)
        .add_plugins(PathFindingPlugin)
        .add_plugins(RockPlugin)
        .add_plugins(PlantsPlugin)
        .add_plugins(InventoryPlugin)
        .add_plugins(ItemDropPlugin)
        .add_plugins(MiscComponentsPlugin)
        .add_plugins(features::input::InputPlugin)
        .add_plugins(features::world_interaction::WorldInteractionPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(PositionPlugin)
        .add_plugins(AiPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(TasksPlugin)
        .add_plugins(ui::UiPlugin)
        .add_plugins(UserActionsPlugin)
        .insert_resource(WorldSeed(555))
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .init_state::<AppState>()
        .add_plugins(
            //#[cfg(debug_assertions)]
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::F1)),
        )
        .run();
}
