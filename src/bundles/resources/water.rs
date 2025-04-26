use crate::bundles::{Id, ItemId, ResourceItem};
use bevy::prelude::*;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
#[require(Id = Id(ItemId::Water),
    Name::new("Water"),
    ResourceItem,
)]
pub struct Water;
