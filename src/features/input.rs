use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<CameraPanAction>::default())
            .add_plugins(InputManagerPlugin::<CameraZoomAction>::default())
            .add_plugins(InputManagerPlugin::<WorldInteractionAction>::default())
            .add_plugins(InputManagerPlugin::<WorldSpeedAction>::default())
            .add_plugins(InputManagerPlugin::<SaveLoadAction>::default());
    }
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum CameraPanAction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum CameraZoomAction {
    In,
    Out,
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum WorldInteractionAction {
    PrimarySelect,
    SecondarySelect,
    PrimaryDragModifier,
    SecondaryDragModifier,
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum WorldSpeedAction {
    TogglePause,
    RealTime,
    Fast,
    Faster,
    Fastest,
}


#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum SaveLoadAction {
    QuickSave,
    QuickLoad,
}
