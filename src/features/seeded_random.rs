use bevy::prelude::*;
use rand::SeedableRng;
use wyrand::WyRand;
use crate::features::map::map_model::WorldSeed;

pub struct SeededRandomPlugin;

#[derive(Resource)]
pub struct RandomSource(pub(crate) WyRand);

impl Plugin for SeededRandomPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RandomSource(WyRand::new(0)));
        app.add_systems(Update, seed_changed);
    }
}

fn seed_changed(world_seed: Res<WorldSeed>, mut random_source: ResMut<RandomSource>) {
    if world_seed.is_changed() {
        random_source.0 = WyRand::new(world_seed.0);
    }
}