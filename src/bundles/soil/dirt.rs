use crate::features::plants::GrowthProvider;
use crate::bundles::HashMap;
use crate::bundles::{Id, ItemId, ResourceItem};
use crate::features::inventory::Inventory;
use bevy::prelude::*;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
#[require(Id(|| Id(ItemId::Dirt)),
    Name(|| "Dirt"),
    ResourceItem,
    Inventory(|| Inventory {
        items: HashMap::from([(ItemId::Water, 100)]),
        ..Default::default()
    }),
    GrowthProvider,
)]
pub struct Dirt;
