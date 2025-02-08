use crate::features::map::map_model::MapData;
use crate::features::path_finding::path_finding::{
    spawn_pathfinding_task, PathFollowFinished, PathFollowResult,
};
use crate::features::position::WorldPosition;
use beet::prelude::{Action, OnRun, OnRunResult, TargetEntity};
use bevy::math::{IVec2, Vec2};
use bevy::prelude::{Commands, Component, Query, Reflect, Res, Trigger};
use crate::bundles::buildables::Buildable;
use crate::features::misc_components::gltf_asset::GltfAnimation;
use crate::features::path_finding::grid::PathingGridResource;

#[derive(Component, Action, Reflect)]
#[require(ContinueRun, Name(|| "BuildAction"))]
#[systems(build_action)]
pub struct BuildAction {
    pub(crate) target: Entity,
}

#[derive(Component, Reflect, Debug)]
pub struct IsBuilding;

#[allow(clippy::too_many_arguments)]
pub fn build_action(
    actions: Query<(Entity, &BuildAction, &TargetEntity), With<Running>>,
    mut buildables: Query<&mut Buildable>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (tree_node_entity, action, target_agent) in actions.iter() {
        commands.entity(target_agent.0).insert_if_new(IsBuilding);
        println!("Building!");
        let mut buildable = buildables.get_mut(action.target).unwrap();
        let build_stat = 1.0;
        buildable.increase_construction_progress(build_stat * time.delta_secs());
        if buildable.finished {
            commands.entity(tree_node_entity).trigger(OnRunResult::success());
            commands.entity(target_agent.0).remove::<IsBuilding>();
        }
    }
    // 
    // commands.entity(target_agent).observe(
    //     move |path_follow_trigger: Trigger<PathFollowFinished>, mut commands: Commands| {
    //         if path_follow_trigger.related_task != Some(trigger_entity) {
    //             return;
    //         }
    // 
    //         match path_follow_trigger.result {
    //             PathFollowResult::Success => {
    //                 commands
    //                     .entity(trigger_entity)
    //                     .trigger(OnRunResult::success());
    //                 println!("GoTo action finished, success!");
    //             }
    //             PathFollowResult::Failure => {
    //                 commands
    //                     .entity(trigger_entity)
    //                     .trigger(OnRunResult::failure());
    //                 println!("GoTo action finished, failure!");
    //             }
    //         }
    //     },
    // );
}
