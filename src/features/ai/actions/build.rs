use crate::bundles::buildables::Buildable;
use crate::features::ai::TargetEntity;
use crate::features::map::map_model::MapData;
use crate::features::misc_components::gltf_asset::GltfAnimation;
use crate::features::path_finding::grid::PathingGridResource;
use crate::features::path_finding::path_finding::{
    spawn_pathfinding_task, PathFollowFinished, PathFollowResult,
};
use crate::features::position::WorldPosition;
use beet::prelude::ContinueRun;
use beet::prelude::*;
use bevy::math::{IVec2, Vec2};
use bevy::prelude::*;

#[derive(Component, Reflect)]
#[require(ContinueRun, Name(|| "BuildAction"))]
pub struct BuildAction {
    pub(crate) target: Entity,
}

#[derive(Component, Reflect, Debug)]
pub struct IsBuilding;

#[allow(clippy::too_many_arguments)]
pub fn build_action(
    actions: Query<(Entity, &BuildAction, &TargetEntity, &Running)>,
    mut buildables: Query<&mut Buildable>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (tree_node_entity, action, target_agent, running) in actions.iter() {
        commands.entity(target_agent.0).insert_if_new(IsBuilding);
        let mut buildable = buildables.get_mut(action.target).unwrap();
        let build_stat = 1.0;
        buildable.increase_construction_progress(build_stat * time.delta_secs());
        if buildable.finished {
            println!("Building is finished");
            running.trigger_result(&mut commands, tree_node_entity, RunResult::Success);
            commands.entity(target_agent.0).remove::<IsBuilding>();
        }
    }
}
