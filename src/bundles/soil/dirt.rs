use crate::bundles::HashMap;
use crate::bundles::{Id, ItemId};
use crate::features::inventory::Inventory;
use crate::features::plants::GrowthProvider;
use bevy::prelude::*;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
#[require(Id(ItemId::Dirt),
    Name::new("Dirt"),
    Inventory = Inventory {
        items: HashMap::from([(ItemId::Nitrogen, 100)]),
        ..Default::default()
    },
    GrowthProvider,
)]
pub struct Dirt;
