use crate::features::health::Health;
use beet::prelude::ContinueRun;
use beet::prelude::*;
use bevy::prelude::*;

#[derive(Component, Reflect)]
#[require(ContinueRun, Name::new("DestructAction"))]
pub struct DestructAction {
    pub(crate) target: Entity,
}

#[derive(Component, Reflect, Debug)]
pub struct IsDestructing;

pub fn destruct_action(
    actions: Query<(Entity, &DestructAction, &Running)>,
    mut healths: Query<&mut Health>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (tree_node_entity, action, running) in actions.iter() {
        println!("Destructing");
        let target_agent = running.origin;
        commands.entity(target_agent).insert_if_new(IsDestructing);
        let mut health = healths.get_mut(action.target).unwrap();
        // TODO: Actual stat
        let settler_destruct_stat = 1.0;
        health.reduce_health(settler_destruct_stat * time.delta_secs());
        if health.health <= 0.0 {
            println!("Health depleted");
            running.trigger_result(&mut commands, tree_node_entity, RunResult::Success);
            commands.entity(target_agent).remove::<IsDestructing>();
        }
    }
}
