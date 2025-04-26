use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use crate::features::states::AppState::InGame;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EnhancedInputPlugin);
        app.add_systems(OnEnter(InGame), |mut commands: Commands| {
            commands.spawn((PlayerInput, Actions::<InGameInputContext>::default(), StateScoped(InGame)));
        });

        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn((PlayerInput, Actions::<OmniPresentInputContext>::default()));
        });

        // app.add_plugins(InputManagerPlugin::<CameraPanAction>::default())
        //     .add_plugins(InputManagerPlugin::<CameraZoomAction>::default())
        //     .add_plugins(InputManagerPlugin::<world_interaction_action>::default())
        //     .add_plugins(InputManagerPlugin::<world_speed_action>::default())
        //     .add_plugins(InputManagerPlugin::<save_load_action>::default());
        app.add_input_context::<InGameInputContext>();
        app.add_input_context::<OmniPresentInputContext>();
    }
}

#[derive(InputContext)]
pub struct InGameInputContext;

#[derive(InputContext)]
pub struct OmniPresentInputContext;

#[derive(Component, Default)]
pub struct PlayerInput;

#[derive(InputAction, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
#[input_action(output = Vec2)]
pub struct CameraPanAction;

#[derive(InputAction, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
#[input_action(output = Vec2)]
pub struct CameraZoomAction;

pub mod world_interaction_action {
    use bevy::prelude::Reflect;
    use bevy_enhanced_input::prelude::InputAction;

    #[derive(InputAction, Clone, Copy, Debug, Reflect)]
    #[input_action(output = bool)]
    pub struct PrimarySelect;

    #[derive(InputAction, Clone, Copy, Debug, Reflect)]
    #[input_action(output = bool)]
    pub struct SecondarySelect;

    #[derive(InputAction, Clone, Copy, Debug, Reflect)]
    #[input_action(output = bool)]
    pub struct PrimaryDragModifier;

    #[derive(InputAction, Clone, Copy, Debug, Reflect)]
    #[input_action(output = bool)]
    pub struct SecondaryDragModifier;
}

pub mod world_speed_action {
    use bevy::prelude::Reflect;
    use bevy_enhanced_input::prelude::InputAction;

    #[derive(InputAction, Clone, Copy, Debug, Reflect)]
    #[input_action(output = bool)]
    pub struct TogglePause;

    #[derive(InputAction, Clone, Copy, Debug, Reflect)]
    #[input_action(output = bool)]
    pub struct RealTime;

    #[derive(InputAction, Clone, Copy, Debug, Reflect)]
    #[input_action(output = bool)]
    pub struct Fast;

    #[derive(InputAction, Clone, Copy, Debug, Reflect)]
    #[input_action(output = bool)]
    pub struct Faster;

    #[derive(InputAction, Clone, Copy, Debug, Reflect)]
    #[input_action(output = bool)]
    pub struct Fastest;
}

pub mod save_load_action {
    use bevy::prelude::Reflect;
    use bevy_enhanced_input::prelude::InputAction;

    #[derive(InputAction, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
    #[input_action(output = bool)]
    pub struct QuickSave;

    #[derive(InputAction, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
    #[input_action(output = bool)]
    pub struct QuickLoad;
}
