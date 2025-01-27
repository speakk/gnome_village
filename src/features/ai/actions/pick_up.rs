use crate::bundles::{Id, ItemStack};
use crate::features::inventory::Inventory;
use beet::prelude::Action;
use bevior_tree::node::NodeResult;
use bevior_tree::prelude::{delegate_node, TaskBridge, TaskEvent, TaskStatus};
use bevy::math::IVec2;
use bevy::prelude::{Component, Entity, In, Query, Reflect};

#[derive(Component, Action, Reflect)]
#[require(Name(|| "PickUpAction"))]
#[observers(pick_up_action)]
pub struct PickUpAction {
    pub target_entity: Entity,
    pub amount: u32,
}

fn pick_up_action(
    trigger: Trigger<OnRun>,
    agents: Query<&TargetEntity>,
    pick_up_requests: Query<&PickUpAction>,
    mut item_stack: Query<&mut ItemStack>,
    mut inventory: Query<&mut Inventory>,
    item_ids: Query<&Id>,
    mut commands: Commands,
) {
    println!("Picking up item, inside pick up action");
    let agent = agents.get(trigger.entity()).unwrap().0;
    let pick_up_request = pick_up_requests.get(trigger.entity()).unwrap();

    let mut inventory = inventory.get_mut(agent).unwrap();
    let target_entity = pick_up_request.target_entity;
    let mut item_stack = item_stack.get_mut(target_entity).unwrap();
    let amount = pick_up_request.amount;

    item_stack.0 -= amount;
    inventory.add_item(**item_ids.get(target_entity).unwrap(), amount);

    //commands.entity(trigger.entity()).remove::<PickUpAction>();
    commands
        .entity(trigger.entity())
        .trigger(OnRunResult::success());
}
