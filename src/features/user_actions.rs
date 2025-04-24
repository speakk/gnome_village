use crate::bundles::{ItemCategory, ItemId};
use crate::ui::ui_main_actions::build_menu::BuildMenuBuildableSelected;
use crate::ui::ui_main_actions::orders_menu::{OrderId, OrderMenuItemSelected};
use bevy::prelude::*;
use crate::ui::ui_main_actions::MainMenuSelectionCleared;

pub type IdFilter = Option<Vec<ItemId>>;
pub type CategoryFilter = Option<Vec<ItemCategory>>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UserActionType {
    Build {
        bundle_type: ItemId,
        coordinates: Vec<IVec2>,
    },
    CancelJobs {
        coordinates: Vec<IVec2>,
        id_filter: IdFilter,
    },
    Destruct {
        coordinates: Vec<IVec2>,
        category_id_filter: CategoryFilter,
    },
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum UserActionState {
    PlacingBuilding(ItemId),
    CancellingJobs(IdFilter),
    Destructing(CategoryFilter),
}

#[derive(Resource, Default)]
pub struct CurrentUserActionState(pub Option<UserActionState>);

#[derive(Event)]
pub struct UserActionIntent(pub UserActionType);

pub struct UserActionsPlugin;

impl Plugin for UserActionsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentUserActionState(None))
            .add_event::<UserActionIntent>()
            .add_systems(
                Update,
                (
                    react_to_buildable_menu_item_selected,
                    react_to_order_item_selected,
                    react_to_menu_cleared
                ),
            );
    }
}

fn react_to_menu_cleared(
    mut event: EventReader<MainMenuSelectionCleared>,
    mut current_action_state: ResMut<CurrentUserActionState>,
) {
    for _event in event.read() {
        current_action_state.0 = None;
    }
}

fn react_to_buildable_menu_item_selected(
    mut build_menu_buildable_selected: EventReader<BuildMenuBuildableSelected>,
    mut current_action_state: ResMut<CurrentUserActionState>,
) {
    for event in build_menu_buildable_selected.read() {
        current_action_state.0 = Some(UserActionState::PlacingBuilding(event.0));
    }
}

fn react_to_order_item_selected(
    mut order_item_selected: EventReader<OrderMenuItemSelected>,
    mut current_action_state: ResMut<CurrentUserActionState>,
) {
    for event in order_item_selected.read() {
        match &event.0 {
            OrderId::Destruct(category_filter) => {
                current_action_state.0 = Some(UserActionState::Destructing(category_filter.clone()));
            }
        }
    }
}
