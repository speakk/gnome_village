pub mod assign_jobs;
pub mod build_task;
pub mod destruct_task;
pub mod water_plants;

use crate::features::misc_components::ItemAmount;
use crate::features::tasks::task::{DepositTarget, RunType, Task};
use bevy::prelude::*;
use crate::features::tasks::sub_tasks::bring_resource_task::{BringResourceRuntimeData, BringResourceTask};

#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct Job;

fn create_bring_resource_task_from_item_amount(
    target_inventory_entity: Entity,
    child_builder: &mut ChildSpawnerCommands,
    item_amount: &ItemAmount,
) {
    for _ in 0..item_amount.amount {
        child_builder.spawn((
            Name::new("BringResource".to_string()),
            BringResourceTask {
                item_requirement: ItemAmount {
                    item_id: item_amount.item_id,
                    amount: 1,
                },
                target: DepositTarget::Inventory(target_inventory_entity),
                run_time_data: BringResourceRuntimeData {
                    concrete_resource_entity: None
                },
            }
        ));
    }
}
