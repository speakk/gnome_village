mod actions;
pub mod trees;

use crate::features::ai::actions::go_to::GoToAction;
use crate::features::ai::actions::pick_up::PickUpAction;
use crate::features::ai::trees::bring_resource::create_bring_resource_tree;
use crate::features::path_finding::Path;
use beet::prelude::ActionPlugin;
use bevy::prelude::*;
use moonshine_core::prelude::MapEntities;
use std::fmt::Debug;
use crate::features::ai::actions::finish_task::FinishTaskAction;

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ActionPlugin::<(GoToAction, PickUpAction, FinishTaskAction)>::default())
            .add_systems(Update, create_bring_resource_tree);
    }
}

#[derive(Component, Reflect)]
pub struct WorkingOnTask(pub Entity);

#[derive(Component)]
pub struct PathFollow {
    pub path: Path,
    pub current_path_index: usize,
    pub finished: bool,
}

impl Default for PathFollow {
    fn default() -> Self {
        PathFollow {
            path: Path {
                steps: vec![],
                related_task: None,
            },
            current_path_index: 0,
            finished: false,
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

impl MapEntities for WorkingOnTask {
    fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
        let entity = &mut self.0;
        *entity = entity_mapper.map_entity(*entity);
    }
}
