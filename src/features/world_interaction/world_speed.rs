use crate::features::input::WorldSpeedAction;
use bevy::app::{App, Startup, Update};
use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;
use leafwing_input_manager::input_map::InputMap;
use leafwing_input_manager::prelude::ButtonlikeChord;
use leafwing_input_manager::InputManagerBundle;

pub struct WorldSpeedPlugin;

impl Plugin for WorldSpeedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, handle_speed_change);
    }
}

fn setup(mut commands: Commands) {
    let input_map = InputMap::new([
        (
            WorldSpeedAction::TogglePause,
            ButtonlikeChord::new([KeyCode::Space]),
        ),
        (
            WorldSpeedAction::RealTime,
            ButtonlikeChord::new([KeyCode::AltLeft, KeyCode::Digit1]),
        ),
        (
            WorldSpeedAction::Fast,
            ButtonlikeChord::new([KeyCode::AltLeft, KeyCode::Digit2]),
        ),
        (
            WorldSpeedAction::Faster,
            ButtonlikeChord::new([KeyCode::AltLeft, KeyCode::Digit3]),
        ),
        (
            WorldSpeedAction::Fastest,
            ButtonlikeChord::new([KeyCode::AltLeft, KeyCode::Digit4]),
        ),
    ]);
    commands.spawn(InputManagerBundle::with_map(input_map));
}

pub fn handle_speed_change(
    query: Query<&ActionState<WorldSpeedAction>>,
    mut virtual_time: ResMut<Time<Virtual>>,
    mut paused: Local<bool>,
) {
    let action_state = query.single();

    if action_state.just_pressed(&WorldSpeedAction::TogglePause) {
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

    if action_state.just_pressed(&WorldSpeedAction::RealTime) {
        virtual_time.set_relative_speed(1.0);
        println!("Speed changed to 1.0");
    }

    if action_state.just_pressed(&WorldSpeedAction::Fast) {
        virtual_time.set_relative_speed(2.0);
        println!("Speed changed to 2.0");
    }

    if action_state.just_pressed(&WorldSpeedAction::Faster) {
        virtual_time.set_relative_speed(3.5);
        println!("Speed changed to 3.5");
    }

    if action_state.just_pressed(&WorldSpeedAction::Fastest) {
        virtual_time.set_relative_speed(6.0);
        println!("Speed changed to 6.0");
    }
}
