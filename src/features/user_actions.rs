use crate::bundles::ItemId;
use crate::ui::ui_main_actions::build_menu::BuildMenuBuildableSelected;
use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UserActionType {
    Build {
        bundle_type: ItemId,
        coordinates: Vec<IVec2>,
    },
    Orders,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum UserActionState {
    #[default]
    None,
    PlacingBuilding,
}

#[derive(Event)]
pub struct UserActionIntent(pub UserActionType);

pub struct UserActionsPlugin;

impl Plugin for UserActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<UserActionState>()
            .add_event::<UserActionIntent>()
            .add_systems(Update, react_to_buildable_menu_selected);
    }
}

fn react_to_buildable_menu_selected(
    mut build_menu_buildable_selected: EventReader<BuildMenuBuildableSelected>,
    mut next_state: ResMut<NextState<UserActionState>>,
) {
    for _ in build_menu_buildable_selected.read() {
        println!("Entering PlacingBuilding state");
        next_state.set(UserActionState::PlacingBuilding);
    }
}
