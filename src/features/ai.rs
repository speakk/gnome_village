pub mod actions;
pub mod trees;
pub mod utility_ai;

use crate::features::ai::actions::ActionsPlugin;
use crate::features::ai::trees::bring_resource::create_bring_resource_tree;
use crate::features::ai::trees::build::create_build_tree;
use crate::features::ai::trees::destruct::create_destruct_tree;
use crate::features::ai::utility_ai::plugin::UtilityAiPlugin;
use crate::features::path_finding::path_finding::Path;
use bevy::prelude::*;
use moonshine_core::prelude::MapEntities;
use moonshine_core::prelude::ReflectMapEntities;
use moonshine_core::prelude::Unload;
use std::fmt::Debug;

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((UtilityAiPlugin, ActionsPlugin))
            .add_systems(
                Update,
                (
                    create_bring_resource_tree,
                    create_build_tree,
                    create_destruct_tree,
                ),
            );
    }
}

#[derive(Component, Reflect)]
pub struct TargetEntity(pub Entity);

#[derive(Component, Reflect)]
#[reflect(Component, MapEntities)]
pub struct WorkingOnTask(pub Entity);

impl MapEntities for WorkingOnTask {
    fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
        let entity = &mut self.0;
        *entity = entity_mapper.get_mapped(*entity);
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
