use crate::bundles::{Id, ItemId, Prototypes};
use crate::features::misc_components::{InWorld, ItemAmount, Prototype};
use crate::features::position::WorldPosition;
use crate::ReflectComponent;
use bevy::prelude::*;
use bevy_platform::collections::HashMap;
use std::cmp::Ordering;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InventoryChanged>()
            .add_observer(update_inventory_amount)
            .add_observer(spawn_public_items)
            .add_observer(remove_public_items)
            .add_systems(Update, emit_inventory_changed_on_spawn);
    }
}

#[derive(Component, Default, Reflect, Debug)]
#[reflect(Component)]
pub struct Inventory {
    pub items: HashMap<ItemId, u32>,
    pub public_container: bool,
}

impl Inventory {
    fn add_item(&mut self, item_id: ItemId, amount: u32) {
        self.items
            .entry(item_id)
            .and_modify(|v| *v += amount)
            .or_insert(amount);
    }

    fn remove_item(&mut self, item_id: ItemId, amount: u32) {
        let current_amount = self.items.get(&item_id).unwrap_or(&0);

        match current_amount.cmp(&amount) {
            Ordering::Greater => {
                self.items.entry(item_id).and_modify(|v| *v -= amount);
            }
            Ordering::Equal => {
                self.items.remove(&item_id);
            }
            Ordering::Less => panic!("Not enough items in inventory"),
        }
    }

    pub fn has_amount(&self, item_id: ItemId, amount: u32) -> bool {
        let current_amount = self.items.get(&item_id).unwrap_or(&0);
        *current_amount >= amount
    }
}

#[derive(Component, Debug, Clone)]
pub struct InInventory(pub Entity);

#[derive(Debug, Clone)]
pub enum InventoryChangedType {
    Add(ItemAmount),
    Remove(ItemAmount),
}

#[derive(Event)]
pub struct InventoryChanged(pub InventoryChangedType);

pub fn emit_inventory_changed_on_spawn(
    query: Query<(Entity, &Inventory), Added<Inventory>>,
    mut commands: Commands,
) {
    for (entity, inventory) in query.iter() {
        for (id, amount) in &inventory.items {
            commands
                .entity(entity)
                .trigger(InventoryChanged(InventoryChangedType::Add(ItemAmount {
                    item_id: *id,
                    amount: *amount,
                })));
        }
    }
}

pub fn spawn_public_items(
    trigger: Trigger<InventoryChanged>,
    inventories: Query<(&Inventory, &WorldPosition)>,
    mut commands: Commands,
    prototypes: Res<Prototypes>,
) {
    let InventoryChangedType::Add(item_amount) = trigger.0 else {
        return;
    };

    let Ok((inventory, world_position)) = inventories.get(trigger.target()) else {
        return;
    };

    if !inventory.public_container {
        return;
    }

    for _i in 0..item_amount.amount {
        let prototype_entity = *prototypes.0.get(&item_amount.item_id).unwrap();
        let new_item = commands
            .entity(prototype_entity)
            .clone_and_spawn()
            .insert((
                InInventory(trigger.target()),
                WorldPosition(world_position.0),
                InWorld,
            ))
            .remove::<Prototype>()
            .id();

        commands.entity(trigger.target()).add_child(new_item);
    }
}

pub fn remove_public_items(
    trigger: Trigger<InventoryChanged>,
    children: Query<&Children>,
    item_ids: Query<&Id>,
    mut commands: Commands,
) {
    let InventoryChangedType::Remove(item_amount) = trigger.0 else {
        return;
    };

    let valid_children = children
        .iter_descendants(trigger.target())
        .filter(|child| {
            if let Ok(item_id) = item_ids.get(*child) {
                if item_id.0 == item_amount.item_id {
                    return true;
                }
            }

            false
        })
        .take(item_amount.amount as usize)
        .collect::<Vec<_>>();

    for child in valid_children {
        commands.entity(child).despawn();
    }
}

pub fn update_inventory_amount(
    trigger: Trigger<InventoryChanged>,
    mut inventories: Query<&mut Inventory>,
) {
    let mut inventory = inventories.get_mut(trigger.target()).unwrap();
    match trigger.0 {
        InventoryChangedType::Add(item_amount) => {
            inventory.add_item(item_amount.item_id, item_amount.amount);
        }
        InventoryChangedType::Remove(item_amount) => {
            inventory.remove_item(item_amount.item_id, item_amount.amount);
        }
    }
}
