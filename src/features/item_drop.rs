use crate::bundles::{ItemId, ItemSpawners};
use crate::features::misc_components::InWorld;
use crate::features::position::WorldPosition;
use crate::features::seeded_random::RandomSource;
use bevy::prelude::*;
use rand::Rng;

pub struct ItemDropPlugin;

impl Plugin for ItemDropPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(item_drop);
    }
}

#[derive(Reflect)]
pub struct SingleItemDrop {
    pub item_id: ItemId,
    pub chance: f32,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ItemDrop {
    pub item_drops: Vec<SingleItemDrop>,
}

fn item_drop(
    trigger: Trigger<OnRemove, ItemDrop>,
    query: Query<(&WorldPosition, &ItemDrop)>,
    mut commands: Commands,
    mut random_source: ResMut<RandomSource>,
    spawners: Res<ItemSpawners>,
) {
    let entity = trigger.target();
    let (world_position, item_drop) = query.get(entity).unwrap();

    for single_item_drop in item_drop.item_drops.iter() {
        if random_source.0.random_range(0.0..1.0) < single_item_drop.chance {
            let new_entity = spawners.0.get(&single_item_drop.item_id).unwrap()(&mut commands);
            commands
                .entity(new_entity)
                .insert((WorldPosition(world_position.0), InWorld));
        }
    }
}
