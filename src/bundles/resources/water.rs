use crate::bundles::{Id, ItemId, ResourceItem};
use bevy::prelude::*;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
#[require(Id(|| Id(ItemId::Water)),
    Name(|| "Water"),
    ResourceItem,
)]
pub struct Water;