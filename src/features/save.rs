use crate::bundles::buildables::torch::WoodenTorch;
use crate::bundles::buildables::wooden_wall::WoodenWall;
use crate::bundles::rock::Rock;
use crate::bundles::settler::Settler;
use crate::features::input::SaveLoadAction;
use crate::features::map::map_model::MapData;
use crate::features::position::WorldPosition;
use bevy::prelude::KeyCode::F8;
use bevy::prelude::*;
use directories::ProjectDirs;
use leafwing_input_manager::action_state::ActionState;
use leafwing_input_manager::input_map::InputMap;
use leafwing_input_manager::InputManagerBundle;
use moonshine_core::load::load;
use moonshine_core::prelude::{file_from_resource, save_default, GetFilePath};
use std::fs;
use std::path::{Path, PathBuf};
use KeyCode::F5;

pub struct SavePlugin;

const SAVE_NAME: &str = "save.ron";

/// A resource which is used to invoke the save system.
#[derive(Resource)]
struct SaveRequest(PathBuf);

impl SaveRequest {
    pub fn new() -> Self {
        if let Some(proj_dirs) = ProjectDirs::from("com", "speak", "rust_village") {
            let path = proj_dirs.config_dir();
            let Ok(_) = fs::create_dir_all(path) else {
                panic!("Could not create save directory");
            };
            SaveRequest(path.join(SAVE_NAME))
        } else {
            panic!("Could not get project directories");
        }
    }
}

impl GetFilePath for SaveRequest {
    fn path(&self) -> &Path {
        &self.0
    }
}

/// A resource which is used to invoke the save system.
#[derive(Resource)]
struct LoadRequest(PathBuf);

impl LoadRequest {
    pub fn new() -> Self {
        if let Some(proj_dirs) = ProjectDirs::from("com", "speak", "rust_village") {
            LoadRequest(proj_dirs.config_dir().join(SAVE_NAME))
        } else {
            panic!("Could not get project directories");
        }
    }
}

impl GetFilePath for LoadRequest {
    fn path(&self) -> &Path {
        &self.0
    }
}

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            moonshine_core::save::SavePlugin,
            moonshine_core::load::LoadPlugin,
        ))
        .register_type::<Settler>()
        .register_type::<WorldPosition>()
        .register_type::<MapData>()
        .register_type::<Rock>()
        .register_type::<WoodenTorch>()
        .register_type::<WoodenWall>()
        .add_systems(Startup, setup)
        .add_systems(
            PreUpdate,
            save_default().into(file_from_resource::<SaveRequest>()),
        )
        .add_systems(PreUpdate, load(file_from_resource::<LoadRequest>()))
        .add_systems(Update, handle_save_input);
    }
}

fn setup(mut commands: Commands) {
    let input_map = InputMap::new([
        (SaveLoadAction::QuickSave, F5),
        (SaveLoadAction::QuickLoad, F8),
    ]);

    commands.spawn(InputManagerBundle::with_map(input_map));
}

fn handle_save_input(mut query: Query<&ActionState<SaveLoadAction>>, mut commands: Commands) {
    for action_state in &mut query {
        if action_state.just_pressed(&SaveLoadAction::QuickSave) {
            commands.insert_resource(SaveRequest::new());
        }

        if action_state.just_pressed(&SaveLoadAction::QuickLoad) {
            commands.insert_resource(LoadRequest::new());
        }
    }
}
