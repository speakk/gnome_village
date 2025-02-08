pub mod actions;
pub mod trees;
pub mod utility_ai;

use crate::features::ai::actions::deposit::DepositAction;
use crate::features::ai::actions::finish_task::FinishTaskAction;
use crate::features::ai::actions::go_to::GoToAction;
use crate::features::ai::actions::pick_up::PickUpAction;
use crate::features::ai::trees::bring_resource::create_bring_resource_tree;
use crate::features::ai::utility_ai::plugin::UtilityAiPlugin;
use crate::features::path_finding::path_finding::Path;
use beet::prelude::ActionPlugin;
use bevy::prelude::*;
use moonshine_core::prelude::MapEntities;
use moonshine_core::prelude::ReflectMapEntities;
use moonshine_core::prelude::Unload;
use std::fmt::Debug;
use crate::features::ai::actions::build::build_action;
use crate::features::ai::trees::build::create_build_tree;

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            UtilityAiPlugin,
            ActionPlugin::<(GoToAction, PickUpAction, FinishTaskAction, DepositAction)>::default(),
        ))
        .add_systems(Update, (create_bring_resource_tree, create_build_tree, build_action));
    }
}

#[derive(Component, Reflect)]
#[reflect(Component, MapEntities)]
pub struct WorkingOnTask(pub Entity);

impl MapEntities for WorkingOnTask {
    fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
        let entity = &mut self.0;
        *entity = entity_mapper.map_entity(*entity);
    }
}

#[derive(Component)]
#[require(Unload)]
pub struct BehaviourTree;

#[derive(Component)]
pub struct PathFollow {
    pub path: Path,
    pub current_path_index: usize,
}

impl Default for PathFollow {
    fn default() -> Self {
        PathFollow {
            path: Path {
                steps: vec![],
                related_task: None,
            },
            current_path_index: 0,
        }
    }
}

impl Debug for PathFollow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PathFollow {{ path: {:?}, current_path_index: {} }}",
            self.path, self.current_path_index
        )
    }
}
