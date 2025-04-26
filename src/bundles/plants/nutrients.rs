use crate::bundles::{Id, ItemId, ResourceItem};
use bevy::prelude::*;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
#[require(Id(ItemId::Nitrogen), Name::new("Nitrogen"), ResourceItem)]
pub struct Nitrogen;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
#[require(Id(ItemId::Potassium), Name::new("Potassium"), ResourceItem)]
pub struct Potassium;
