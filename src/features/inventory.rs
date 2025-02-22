use crate::bundles::ItemId;
use crate::ReflectComponent;
use bevy::prelude::*;
use bevy::utils::HashMap;
use std::cmp::Ordering;
use crate::features::misc_components::InWorld;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InventoryChanged>()
            .add_systems(Update, trigger_inventory_changed);
    }
}

#[derive(Component, Default, Reflect, Debug, Clone)]
#[reflect(Component)]
pub struct Inventory {
    pub items: HashMap<ItemId, u32>,
}

impl Inventory {
    pub fn add_item(&mut self, item_id: ItemId, amount: u32) {
        self.items
            .entry(item_id)
            .and_modify(|v| *v += amount)
            .or_insert(amount);
    }

    pub fn remove_item(&mut self, item_id: ItemId, amount: u32) {
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
        current_amount >= &amount
    }
}

#[derive(Event)]
pub struct InventoryChanged;

pub fn trigger_inventory_changed(
    query: Query<(Entity, &Inventory), (Changed<Inventory>, With<InWorld>)>,
    mut commands: Commands,
) {
    for (entity, inventory) in query.iter() {
        commands.entity(entity).trigger(InventoryChanged);
    }
}
