use crate::bundles::HashMap;
use crate::features::inventory::Inventory;
use crate::bundles::{Id, ItemId, ResourceItem};
use bevy::prelude::*;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
#[require(Id(|| Id(ItemId::Dirt)),
    Name(|| "Dirt"),
    ResourceItem,
    Inventory(|| Inventory {
        items: HashMap::from([(ItemId::Water, 100)])
    })
)]
pub struct Dirt;