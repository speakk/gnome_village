use bevy::prelude::*;
use crate::bundles::buildables::BluePrint;
use crate::features::misc_components::InWorld;
use crate::features::tasks::task::{RunType, Task, TaskType};

pub fn react_to_blueprints(mut commands: Commands, new_blueprints_query: Query<&BluePrint, (Added<BluePrint>, With<InWorld>)>) {
    for blueprint in new_blueprints_query.iter() {
        println!("Got blueprint: {:?}", blueprint);
        let new_task = commands.spawn((Task {
            run_type: RunType::Sequence,
            ..default()
        },)).with_children(|parent_task| {
           let bring_resources = parent_task.spawn((Task {
               run_type: RunType::Parallel,
               ..default()
           }, )).with_children(|bring_resource_task| {
               // Here check building requirements for BluePrint / Constructable1
               bring_resource_task.spawn((Task {
                   run_type: RunType::Leaf,
                   task_type: Some(TaskType::BringResource),
                   ..default()
               },));
           });
        });
    }
}