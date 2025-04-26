use crate::features::input::{InGameInputContext, world_speed_action};
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

pub struct WorldSpeedPlugin;

impl Plugin for WorldSpeedPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(binding)
            .add_observer(toggle_pause_handler)
            .add_observer(set_speed_realtime)
            .add_observer(set_speed_fast)
            .add_observer(set_speed_faster)
            .add_observer(set_speed_fastest);
    }
}

fn binding(
    trigger: Trigger<Binding<InGameInputContext>>,
    mut in_game_input_context: Query<&mut Actions<InGameInputContext>>,
) {
    let mut actions = in_game_input_context.get_mut(trigger.entity()).unwrap();

    actions
        .bind::<world_speed_action::TogglePause>()
        .to((KeyCode::Space,));

    actions
        .bind::<world_speed_action::RealTime>()
        .to((KeyCode::Digit1.with_mod_keys(ModKeys::ALT),));

    actions
        .bind::<world_speed_action::Fast>()
        .to((KeyCode::Digit2.with_mod_keys(ModKeys::ALT),));

    actions
        .bind::<world_speed_action::Faster>()
        .to((KeyCode::Digit3.with_mod_keys(ModKeys::ALT),));

    actions
        .bind::<world_speed_action::Fastest>()
        .to((KeyCode::Digit4.with_mod_keys(ModKeys::ALT),));
}

pub fn toggle_pause_handler(
    _trigger: Trigger<Fired<world_speed_action::TogglePause>>,
    mut virtual_time: ResMut<Time<Virtual>>,
    mut paused: Local<bool>,
) {
    *paused = !*paused;
    // TODO: Maybe resume speed that existed previous
    if *paused {
        virtual_time.set_relative_speed(0.0);
        println!("Paused, speed 0");
    } else {
        virtual_time.set_relative_speed(1.0);
        println!("Resumed, speed 1");
    }
}

pub fn set_speed_realtime(
    _trigger: Trigger<Fired<world_speed_action::RealTime>>,
    mut virtual_time: ResMut<Time<Virtual>>,
) {
    virtual_time.set_relative_speed(1.0);
    println!("Speed changed to 1.0");
}

pub fn set_speed_fast(
    _trigger: Trigger<Fired<world_speed_action::Fast>>,
    mut virtual_time: ResMut<Time<Virtual>>,
) {
    virtual_time.set_relative_speed(2.0);
    println!("Speed changed to 2.0");
}

pub fn set_speed_faster(
    _trigger: Trigger<Fired<world_speed_action::Faster>>,
    mut virtual_time: ResMut<Time<Virtual>>,
) {
    virtual_time.set_relative_speed(3.5);
    println!("Speed changed to 3.5");
}

pub fn set_speed_fastest(
    _trigger: Trigger<Fired<world_speed_action::Fastest>>,
    mut virtual_time: ResMut<Time<Virtual>>,
) {
    virtual_time.set_relative_speed(6.0);
    println!("Speed changed to 6.0");
}
