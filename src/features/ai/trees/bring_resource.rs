use crate::bundles::settler::Settler;
use crate::bundles::{Id, Reservation, Reservations, ResourceItem};
use crate::features::ai::actions::deposit::DepositAction;
use crate::features::ai::actions::finish_task::FinishTaskAction;
use crate::features::ai::actions::go_to::GoToAction;
use crate::features::ai::actions::pick_up::PickUpAction;
use crate::features::ai::{BehaviourTree, WorkingOnTask};
use crate::features::inventory::Inventory;
use crate::features::misc_components::InWorld;
use crate::features::position::WorldPosition;
use crate::features::tasks::task::{BringResourceData, BringResourceRuntimeData, DepositTarget, Task, TaskCancelled, TaskType};
use beet::prelude::{OnRun, SequenceFlow, TargetEntity};
use bevy::prelude::*;

pub fn create_bring_resource_tree(
    work_started_query: Query<(&WorkingOnTask, Entity), Added<WorkingOnTask>>,
    item_resources: Query<&WorldPosition, (With<ResourceItem>, With<InWorld>)>,
    inventories: Query<&WorldPosition, (With<Inventory>, With<InWorld>)>,
    tasks: Query<&Task>,
    mut commands: Commands,
) {
    for (working_on_task, worker_entity) in work_started_query.iter() {
        let task = tasks.get(working_on_task.0).unwrap();
        println!("Found WorkingOnTask");

        if let Some(TaskType::BringResource(bring_resource_data)) = &task.task_type {
            println!("Had BringResource task, creating tree");
            let target_coordinate = match bring_resource_data.target {
                DepositTarget::Coordinate(coordinate) => coordinate,
                DepositTarget::Inventory(inventory_entity) => {
                    inventories.get(inventory_entity).unwrap().0.as_ivec2()
                }
            };

            let resource_target = bring_resource_data
                .run_time_data
                .unwrap()
                .concrete_resource_entity;
            let resource_position = item_resources.get(resource_target);

            // TODO: Make mechanism to clean up in case Settler gets despawned
            let tree_entity = commands
                .spawn((BehaviourTree, SequenceFlow))
                .with_children(|root| {
                    println!("Creating tree, spawning goto");

                    // TODO: Right now using the existence of resource_position as an indicator
                    // that we already picked this resource up. (In case creating tree from save game)
                    // Rather check Inventory for the item
                    if let Ok(resource_position) = resource_position {
                        root.spawn((
                            GoToAction {
                                target: resource_position.0.as_ivec2(),
                            },
                            TargetEntity(worker_entity),
                        ));

                        root.spawn((
                            PickUpAction {
                                target_entity: resource_target,
                                amount: bring_resource_data.item_requirement.amount,
                            },
                            TargetEntity(worker_entity),
                        ));
                    }

                    root.spawn((
                        GoToAction {
                            target: target_coordinate,
                        },
                        TargetEntity(worker_entity),
                    ));

                    root.spawn((
                        DepositAction {
                            deposit_target: bring_resource_data.target,
                            amount: bring_resource_data.item_requirement.amount,
                            item_id: bring_resource_data.item_requirement.item_id,
                        },
                        TargetEntity(worker_entity),
                    ));

                    root.spawn((
                        FinishTaskAction {
                            task: working_on_task.0,
                            tree_root: root.parent_entity(),
                        },
                        TargetEntity(worker_entity),
                    ));
                })
                .trigger(OnRun).id();

            commands.entity(working_on_task.0).observe(move |_trigger: Trigger<TaskCancelled>, mut commands: Commands| {
                commands.entity(tree_entity).try_despawn_recursive();
            });
        }
    }
}

pub fn score_bring_resource(
    resources_query: &mut Query<
        (Entity, &WorldPosition, &Id, &mut Reservations),
        (With<ResourceItem>, With<InWorld>),
    >,
    agents: &Vec<(Entity, &WorldPosition)>,
    bring_resource_data: &mut BringResourceData,
    others_query: &Query<(Entity, &WorldPosition), (Without<ResourceItem>, Without<Settler>)>,
) -> Option<Entity> {
    println!("Scoring bring resource");
    let mut best_resource_entity: Option<Entity> = None;
    let mut best_agent: Option<Entity> = None;
    let mut best_score = -9999.0;

    let target = match bring_resource_data.target {
        DepositTarget::Coordinate(coordinate) => coordinate,
        DepositTarget::Inventory(inventory_entity) => {
            others_query.get(inventory_entity).unwrap().1.as_ivec2()
        }
    };

    let valid_resources = resources_query
        .iter()
        .filter(|(_, _, id, reservations)| {
            let reserved_amount = reservations.0.iter().map(|r| r.amount).sum::<u32>();
            id.0 == bring_resource_data.item_requirement.item_id
                && reserved_amount < bring_resource_data.item_requirement.amount
        })
        .collect::<Vec<_>>();

    for (resource_entity, resource_position, _id, _reservations) in valid_resources.iter() {
        for (agent_entity, agent_position) in agents.iter() {
            let agent_to_resource_distance = resource_position.0.distance(agent_position.0);
            let resource_to_goal_distance = target.as_vec2().distance(resource_position.0);
            let score = -agent_to_resource_distance - resource_to_goal_distance; // Smaller distance is better
            if score > best_score {
                best_resource_entity = Some(*resource_entity);
                best_agent = Some(*agent_entity);
                best_score = score;
            }
        }
    }

    println!("Best resource entity result: {:?}", best_resource_entity);

    if let Some(resource_entity) = best_resource_entity {
        bring_resource_data.run_time_data = Some(BringResourceRuntimeData {
            concrete_resource_entity: resource_entity,
        });

        resources_query
            .get_mut(resource_entity)
            .unwrap()
            .3
             .0
            .push(Reservation {
                reserved_by: best_agent.unwrap(),
                amount: bring_resource_data.item_requirement.amount,
            });

        return best_agent;
    }

    None
}
