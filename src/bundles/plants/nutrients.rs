use crate::bundles::{Id, ItemId, ResourceItem};
use bevy::prelude::*;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
#[require(Id(|| Id(ItemId::Nitrogen)), Name(|| "Nitrogen"), ResourceItem, )]
pub struct Nitrogen;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
#[require(Id(|| Id(ItemId::Potassium)), Name(|| "Potassium"), ResourceItem, )]
pub struct Potassium;