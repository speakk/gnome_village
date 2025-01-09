use bevy::prelude::*;
use moonshine_core::prelude::Save;
use crate::bundles::buildables::Buildable;
use crate::features::path_finding::Solid;
use crate::features::position::WorldPosition;

#[derive(Component, Default)]
#[require(WorldPosition, Solid, Name(|| "Wooden Wall"), Buildable, Save)]
pub struct Wall;
