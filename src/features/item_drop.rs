use crate::bundles::{ItemId, Prototypes};
use crate::features::misc_components::{InWorld, Prototype};
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
    prototypes: Res<Prototypes>,
) {
    let entity = trigger.target();
    let Ok((world_position, item_drop)) = query.get(entity) else {
        return;
    };

    for single_item_drop in item_drop.item_drops.iter() {
        if random_source.0.random_range(0.0..1.0) < single_item_drop.chance {
            commands
                .entity(*prototypes.0.get(&single_item_drop.item_id).unwrap())
                .clone_and_spawn()
                .insert((WorldPosition(world_position.0), InWorld))
                .remove::<Prototype>();
        }
    }
}
