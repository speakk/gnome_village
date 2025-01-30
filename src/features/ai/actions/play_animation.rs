use crate::bundles::{ItemStack};
use beet::prelude::Action;
use bevy::prelude::{Component, Query, Reflect};
use crate::features::misc_components::gltf_asset::GltfAnimation;

#[derive(Component, Action, Reflect)]
#[require(Name(|| "PlayAnimationAction"))]
#[observers(play_animation_action)]
pub struct PlayAnimationAction {
    pub animation_index: usize,
}

fn play_animation_action(
    trigger: Trigger<OnRun>,
    agents: Query<&TargetEntity>,
    actions: Query<&PlayAnimationAction>,
    mut gltf_animations: Query<&mut GltfAnimation>,
    mut commands: Commands
) {
    println!("Picking up item, inside pick up action");
    let agent = agents.get(trigger.entity()).unwrap().0;
    let action = actions.get(trigger.entity()).unwrap();
    
    let mut gltf_animation = gltf_animations.get_mut(agent).unwrap();
    gltf_animation.current_animation_index = action.animation_index;

    commands
        .entity(trigger.entity())
        .trigger(OnRunResult::success());
}
