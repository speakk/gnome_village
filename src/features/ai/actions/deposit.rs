use crate::bundles::{ItemId, ItemSpawners};
use crate::features::inventory::{Inventory, InventoryChanged, InventoryChangedType};
use crate::features::misc_components::ItemAmount;
use crate::features::position::WorldPosition;
use crate::features::tasks::task::DepositTarget;
use beet::prelude::*;
use bevy::prelude::*;

#[action(deposit_action)]
#[derive(Component, Reflect)]
#[require(Name(|| "DepositAction"))]
pub struct DepositAction {
    pub deposit_target: DepositTarget,
    pub item_id: ItemId,
    pub amount: u32,
}

fn deposit_action(
    trigger: Trigger<OnRun>,
    actions: Query<&DepositAction>,
    mut commands: Commands,
    item_spawners: Res<ItemSpawners>,
) {
    println!("Picking up item, inside pick up action");
    let agent = trigger.origin;
    let action = actions.get(trigger.action).unwrap();
    let amount = action.amount;

    commands
        .entity(agent)
        .trigger(InventoryChanged(InventoryChangedType::Remove(ItemAmount {
            item_id: action.item_id,
            amount,
        })));

    match action.deposit_target {
        DepositTarget::Inventory(inventory_entity) => {
            commands
                .entity(inventory_entity)
                .trigger(InventoryChanged(InventoryChangedType::Add(ItemAmount {
                    item_id: action.item_id,
                    amount,
                })));
        }
        DepositTarget::Coordinate(coordinate) => {
            let new_item = item_spawners.0.get(&action.item_id).unwrap()(&mut commands);
            commands
                .entity(new_item)
                .insert(WorldPosition(coordinate.as_vec2()));
        }
    }

    trigger.trigger_result(&mut commands, RunResult::Success);
}
