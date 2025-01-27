use crate::bundles::ItemId;
use crate::features::tasks::task::ItemAmount;
use bevy::prelude::{Component, Reflect};
use bevy::utils::HashMap;
use std::cmp::Ordering;

#[derive(Component, Default, Reflect, Debug, Clone)]
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
