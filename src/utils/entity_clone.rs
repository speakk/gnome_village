use bevy::asset::Assets;
use bevy::prelude::*;

pub trait CloneEntityCommandsExt {
    fn clone_entity(&mut self, entity: Entity) -> Entity;
}

impl<'w, 's> CloneEntityCommandsExt for Commands<'w, 's> {
    fn clone_entity(&mut self, entity: Entity) -> Entity {
        let mut new_entity: Entity = self.spawn_empty().id();
        println!("Cloning entity, id: {:?}", new_entity);
        self.queue(move |world: &mut World| {
            clone_entity(world, entity.clone(), &mut new_entity);
        });
        
        println!("Returning id: {:?}", new_entity);
        new_entity
    }
}

// pub struct CloneEntityCommand {}
// 
// impl Command for CloneEntityCommand {
//     fn apply(self, world: &mut World) {
//         let cloned = clone_entity(world, self.entity);
//         //self.entity = cloned;
//     }
// }

pub fn clone_entity(world: &mut World, entity: Entity, into_entity: &mut Entity) {
    let mut scene_spawner = SceneSpawner::default();
    let scene = DynamicSceneBuilder::from_world(world)
        .extract_entity(entity)
        .build();

    let scene_id = world.resource_mut::<Assets<DynamicScene>>().add(scene);
    let instance_id = scene_spawner.spawn_dynamic_sync(world, &scene_id).unwrap();

    let new_entity = scene_spawner
        .iter_instance_entities(instance_id)
        .next()
        .unwrap();

    *into_entity = new_entity;
}
