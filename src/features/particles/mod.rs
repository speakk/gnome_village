pub mod light_sparkle;

use bevy::prelude::*;
use bevy_platform::collections::HashMap;
use bevy_hanabi::EffectAsset;
use crate::features::particles::light_sparkle::setup_light_sparkle;

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_light_sparkle);
        app.insert_resource(ParticleHandles::default());
    }
}

#[derive(Eq, Hash, PartialEq)]
pub enum ParticleType {
    LightSparkle,
}

#[derive(Resource, Default)]
pub struct ParticleHandles(pub HashMap<ParticleType, Handle<EffectAsset>>);