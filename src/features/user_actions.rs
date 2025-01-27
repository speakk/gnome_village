use bevy::prelude::*;
use crate::ui::ui_main_actions::build_menu::BuildMenuBuildableSelected;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UserActionType {
    Build,
    Orders,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum UserActionState {
    #[default]
    None,
    PlacingBuilding,
}

pub struct UserActionsPlugin;

impl Plugin for UserActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<UserActionState>();
    }
}

fn react_to_buildable_menu_selected(mut build_menu_buildable_selected: EventReader<BuildMenuBuildableSelected>,
                                    mut next_state: ResMut<NextState<UserActionState>>
) {
    for event in build_menu_buildable_selected.read() {
        
        next_state.set(UserActionState::PlacingBuilding);
    }
}