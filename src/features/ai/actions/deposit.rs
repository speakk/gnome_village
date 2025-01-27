use crate::bundles::{Id, ItemId, ItemSpawners, ItemStack};
use crate::features::inventory::Inventory;
use beet::prelude::Action;
use bevy::prelude::{Component, Entity, Query, Reflect};
use crate::features::position::WorldPosition;
use crate::features::tasks::task::DepositTarget;

#[derive(Component, Action, Reflect)]
#[require(Name(|| "DepositAction"))]
#[observers(deposit_action)]
pub struct DepositAction {
    pub deposit_target: DepositTarget,
    pub item_id: ItemId,
    pub amount: u32,
}

fn deposit_action(
    trigger: Trigger<OnRun>,
    agents: Query<&TargetEntity>,
    actions: Query<&DepositAction>,
    mut inventories: Query<&mut Inventory>,
    mut commands: Commands,
    item_spawners: Res<ItemSpawners>
) {
    println!("Picking up item, inside pick up action");
    let agent = agents.get(trigger.entity()).unwrap().0;
    let action = actions.get(trigger.entity()).unwrap();
    let amount = action.amount;

    {
        let mut source_inventory = inventories.get_mut(agent).unwrap();
        source_inventory.remove_item(action.item_id, amount);
    }
    
    match action.deposit_target {
        DepositTarget::Inventory(inventory_entity) => {
            let mut target_inventory = inventories.get_mut(inventory_entity).unwrap();
            target_inventory.add_item(action.item_id, amount);
        },
        DepositTarget::Coordinate(coordinate) => {
            let new_item = item_spawners.0.get(&action.item_id).unwrap()(&mut commands);
            commands.entity(new_item).insert(WorldPosition(coordinate.as_vec2()));
        }
    }
    
    commands
        .entity(trigger.entity())
        .trigger(OnRunResult::success());
}
