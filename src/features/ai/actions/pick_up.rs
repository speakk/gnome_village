use crate::bundles::{Id, ItemStack};
use crate::features::inventory::{InventoryChanged, InventoryChangedType};
use crate::features::misc_components::ItemAmount;
use beet::prelude::*;
use bevy::prelude::*;

#[action(pick_up_action)]
#[derive(Component, Reflect)]
#[require(Name::new("PickUpAction"))]
pub struct PickUpAction {
    pub target_entity: Entity,
    pub amount: u32,
}

fn pick_up_action(
    trigger: Trigger<OnRun>,
    actions: Query<&PickUpAction>,
    mut item_stack: Query<&mut ItemStack>,
    item_ids: Query<&Id>,
    mut commands: Commands,
) {
    println!("Picking up item, inside pick up action");
    let agent = trigger.origin;
    let action = actions.get(trigger.action).unwrap();

    let target_entity = action.target_entity;
    let mut item_stack = item_stack.get_mut(target_entity).unwrap();
    let amount = action.amount;

    item_stack.0 -= amount;
    commands
        .entity(agent)
        .trigger(InventoryChanged(InventoryChangedType::Add(ItemAmount {
            item_id: **item_ids.get(target_entity).unwrap(),
            amount,
        })));

    trigger.trigger_result(&mut commands, RunResult::Success);
}
