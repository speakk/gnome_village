use crate::features::ai::actions::go_to::GoTo;
use crate::features::ai::WorkingOnTask;
use crate::features::tasks::task::{DepositTarget, Task, TaskType};
use bevior_tree::prelude::Sequence;
use bevior_tree::BehaviorTreeBundle;
use bevy::prelude::*;

pub fn create_bring_resource_tree(
    work_started_query: Query<(&WorkingOnTask, Entity), Added<WorkingOnTask>>,
    tasks: Query<&Task>,
    mut commands: Commands,
) {
    for (working_on_task, worker_entity) in work_started_query.iter() {
        let task = tasks.get(working_on_task.0).unwrap();
        println!("Found WorkingOnTask");
        
        if let Some(task_type) = task.task_type {
            println!("Task type: {:?}", task_type);
            if let TaskType::BringResource(bring_resource_data) = task_type {
                println!("Had BringResource task, creating tree");
                let target_coordinate = match bring_resource_data.target {
                    DepositTarget::Coordinate(coordinate) => coordinate,
                    DepositTarget::Inventory(inventory_entity) => panic!(
                        "Inventory target is not supported yet. Inventory entity: {}",
                        inventory_entity
                    )
                };
                
                commands
                    .entity(worker_entity)
                    .insert(BehaviorTreeBundle::from_root(
                        Sequence::new(vec![
                            Box::new(GoTo::new(target_coordinate))
                        ]),
                    ));
            }
        }
    }
}