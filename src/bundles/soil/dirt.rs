use crate::features::plants::GrowthProvider;
use crate::bundles::HashMap;
use crate::bundles::{Id, ItemId, ResourceItem};
use crate::features::inventory::Inventory;
use bevy::prelude::*;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
#[require(Id(|| Id(ItemId::Dirt)),
    Name(|| "Dirt"),
    Inventory(|| Inventory {
        items: HashMap::from([(ItemId::Nitrogen, 100)]),
        ..Default::default()
    }),
    GrowthProvider,
)]
pub struct Dirt;
