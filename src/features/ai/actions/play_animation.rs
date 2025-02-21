use crate::features::misc_components::gltf_asset::GltfAnimation;
use beet::prelude::*;
use bevy::prelude::*;

#[action(play_animation_action)]
#[derive(Component, Reflect)]
#[require(Name(|| "PlayAnimationAction"))]
pub struct PlayAnimationAction {
    pub animation_index: usize,
}

fn play_animation_action(
    trigger: Trigger<OnRun>,
    actions: Query<&PlayAnimationAction>,
    mut gltf_animations: Query<&mut GltfAnimation>,
    mut commands: Commands,
) {
    println!("Picking up item, inside pick up action");
    let agent = trigger.origin;
    let action = actions.get(trigger.action).unwrap();

    let mut gltf_animation = gltf_animations.get_mut(agent).unwrap();
    gltf_animation.current_animation_index = action.animation_index;

    trigger.trigger_result(&mut commands, RunResult::Success);
}
