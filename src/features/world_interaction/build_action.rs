use bevy::prelude::*;
use crate::features::user_actions::UserActionState;
use crate::ui::ui_main_actions::build_menu::BuildMenuBuildableSelected;

struct BuildActionPlugin;

impl Plugin for BuildActionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, react_to_buildable_menu_selected)
            .add_systems(OnEnter(UserActionState::PlacingBuilding), add_building_preview);
    }
}

fn react_to_buildable_menu_selected(mut build_menu_buildable_selected: EventReader<BuildMenuBuildableSelected>, mut current_building: Local<Option<Entity>>) {
    for event in build_menu_buildable_selected.read() {
        *current_building = Some(event.0);
    }
}

fn add_building_preview() {
    
}