use crate::bundles::buildables::Buildable;
use beet::prelude::ContinueRun;
use beet::prelude::*;
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
    actions: Query<(Entity, &BuildAction, &Running)>,
    mut buildables: Query<&mut Buildable>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (tree_node_entity, action, running) in actions.iter() {
        let target_agent = running.origin;
        commands.entity(target_agent).insert_if_new(IsBuilding);
        let mut buildable = buildables.get_mut(action.target).unwrap();
        let build_stat = 1.0;
        buildable.increase_construction_progress(build_stat * time.delta_secs());
        if buildable.finished {
            println!("Building is finished");
            running.trigger_result(&mut commands, tree_node_entity, RunResult::Success);
            commands.entity(target_agent).remove::<IsBuilding>();
        }
    }
}
