use crate::bundles::buildables::BuildablesPlugin;
use crate::bundles::settler::Settler;
use crate::features::misc_components::Prototype;
use bevy::prelude::*;
use moonshine_core::save::Save;
use moonshine_view::RegisterView;

pub mod buildables;
pub mod rock;
pub mod settler;

pub struct BundlePlugin;

impl Plugin for BundlePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BuildablesPlugin).add_viewable::<Settler>();
    }
}


pub fn clone_entity(mut world: &mut World, entity: Entity) -> Entity {
    let mut scene_spawner = SceneSpawner::default();
    let scene = DynamicSceneBuilder::from_world(world)
        .extract_entity(entity)
        .build();

    let scene_id = world.resource_mut::<Assets<DynamicScene>>().add(scene);
    let instance_id = scene_spawner
        .spawn_dynamic_sync(world, &scene_id)
        .unwrap();

    let new_entity = scene_spawner
        .iter_instance_entities(instance_id)
        .next()
        .unwrap();

    new_entity
}


pub fn make_concrete_from_prototype(prototype: Entity, world: &mut World) -> Entity {
    let cloned = clone_entity(world, prototype);
    let mut commands = world.commands();
    commands
        .entity(cloned)
        .insert(Save)
        .remove::<Prototype>()
        .id()
}
