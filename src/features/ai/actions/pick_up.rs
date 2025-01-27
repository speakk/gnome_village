use crate::bundles::{Id, ItemStack};
use crate::features::inventory::Inventory;
use bevior_tree::node::NodeResult;
use bevior_tree::prelude::{delegate_node, TaskBridge, TaskEvent, TaskStatus};
use bevy::prelude::{Entity, In, Query};

#[delegate_node(delegate)]
pub struct PickUp {
    delegate: TaskBridge,
}

const TARGET_DISTANCE_THRESHOLD: f32 = 1.5;

impl PickUp {
    pub fn new(target_entity: Entity, amount: u32) -> Self {
        let checker =
            move |agent: In<Entity>, inventory: Query<&Inventory>, item_ids: Query<&Id>| {
                let inventory = inventory.get(agent.0).unwrap();
                if inventory.has_amount(item_ids.get(target_entity).unwrap().0, amount) {
                    return TaskStatus::Complete(NodeResult::Success);
                }

                TaskStatus::Complete(NodeResult::Failure)
            };

        let task = TaskBridge::new(checker).on_event(
            TaskEvent::Enter,
            move |entity: In<Entity>,
                  mut inventory: Query<&mut Inventory>,
                  mut item_stack: Query<&mut ItemStack>,
                  item_ids: Query<&Id>| {
                let mut inventory = inventory.get_mut(entity.0).unwrap();
                let mut item_stack = item_stack.get_mut(target_entity).unwrap();

                item_stack.0 -= amount;
                inventory.add_item(**item_ids.get(target_entity).unwrap(), amount);

                //println!("Ensure path entered, to {}", target_coordinate);
                //let target_position = WorldPosition(Vec2::new(target_coordinate.x as f32, target_coordinate.y as f32));
                //spawn_pathfinding_task(&mut commands, *entity, &pathing_grid, map_data.single(), *world_position.get(*entity).unwrap(), target_position);
            },
        );

        Self { delegate: task }
    }
}
