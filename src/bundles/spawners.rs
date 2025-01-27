use crate::bundles::buildables::torch::WoodenTorch;
use crate::bundles::buildables::wooden_wall::WoodenWall;
use crate::bundles::resources::lumber::Lumber;
use crate::bundles::settler::Settler;
use crate::bundles::{ItemId, ItemSpawners};
use bevy::prelude::ResMut;

pub fn setup_spawners(mut spawners: ResMut<ItemSpawners>) {
    spawners.insert(ItemId::WoodenTorch, |commands| {
        commands.spawn((WoodenTorch,)).id()
    });

    spawners.insert(ItemId::WoodenWall, |commands| {
        commands.spawn((WoodenWall,)).id()
    });

    spawners.insert(ItemId::Settler, |commands| {
        commands.spawn((Settler::default(),)).id()
    });

    spawners.insert(ItemId::Lumber, |commands| commands.spawn((Lumber,)).id());
}
