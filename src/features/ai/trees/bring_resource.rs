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
use crate::features::tasks::task::{DepositTarget, ResourceFilter, ResourceQuery, TaskCancelled, TaskType};
use beet::prelude::*;
use bevy::prelude::*;
use crate::features::tasks::sub_tasks::bring_resource_task::{BringResourceRuntimeData, BringResourceTask};

pub fn create_bring_resource_tree(
    work_started_query: Query<(&WorkingOnTask, Entity), Added<WorkingOnTask>>,
    item_resources: Query<&WorldPosition, (With<ResourceItem>, With<InWorld>)>,
    inventories: Query<&WorldPosition, (With<Inventory>, With<InWorld>)>,
    tasks: Query<&BringResourceTask>,
    mut commands: Commands,
) {
    for (working_on_task, worker_entity) in work_started_query.iter() {
        let task = tasks.get(working_on_task.0);
        println!("Found WorkingOnTask");

        if let Ok(task) = task {
            println!("Had BringResource task, creating tree");
            let target_coordinate = match task.target {
                DepositTarget::Coordinate(coordinate) => coordinate,
                DepositTarget::Inventory(inventory_entity) => {
                    let id = inventory_entity.to_bits();
                    inventories.get(inventory_entity).unwrap().as_coordinate()
                }
            };

            let resource_target = task.run_time_data.unwrap().concrete_resource_entity;
            let resource_position = item_resources.get(resource_target);

            // TODO: Make mechanism to clean up in case Settler gets despawned
            let tree_entity = commands
                .spawn((BehaviourTree, Sequence))
                .with_children(|root| {
                    println!("Creating tree, spawning goto");

                    // TODO: Right now using the existence of resource_position as an indicator
                    // that we already picked this resource up. (In case creating tree from save game)
                    // Rather check Inventory for the item
                    if let Ok(resource_position) = resource_position {
                        root.spawn((GoToAction {
                            target: resource_position.as_coordinate(),
                        },));

                        root.spawn((PickUpAction {
                            target_entity: resource_target,
                            amount: task.item_requirement.amount,
                        },));
                    }

                    root.spawn((GoToAction {
                        target: target_coordinate,
                    },));

                    root.spawn((DepositAction {
                        deposit_target: task.target,
                        amount: task.item_requirement.amount,
                        item_id: task.item_requirement.item_id,
                    },));

                    root.spawn((FinishTaskAction {
                        task: working_on_task.0,
                        tree_root: root.target_entity(),
                    },));
                })
                .id();

            commands
                .entity(tree_entity)
                .trigger(OnRunAction::new(tree_entity, worker_entity, ()));

            commands.entity(working_on_task.0).observe(
                move |_trigger: Trigger<TaskCancelled>, mut commands: Commands| {
                    commands.entity(tree_entity).despawn_recursive();
                    println!("Despawned tree_entity?");
                },
            );
        }
    }
}

impl TaskType for BringResourceTask {
    fn score(
        &mut self,
        mut resources: &mut Query<
            ResourceQuery,
            ResourceFilter
        >,
        agents: &[(Entity, &WorldPosition)],
        others_query: &Query<(Entity, &WorldPosition), (Without<ResourceItem>, Without<Settler>)>,
    ) -> Option<Entity> {
        let mut best_resource_entity: Option<Entity> = None;
        let mut best_agent: Option<Entity> = None;
        let mut best_score = -9999.0;

        let target = match self.target {
            DepositTarget::Coordinate(coordinate) => coordinate,
            DepositTarget::Inventory(inventory_entity) => others_query
                .get(inventory_entity)
                .unwrap()
                .1
                .as_coordinate(),
        };

        let valid_resources = resources
            .iter()
            .filter(|(query_item)| {
                let reserved_amount = query_item.reservations.0.iter().map(|r| r.amount).sum::<u32>();
                query_item.id.0 == self.item_requirement.item_id
                    && reserved_amount < self.item_requirement.amount
            })
            .collect::<Vec<_>>();

        for query_item in valid_resources.iter() {
            for (agent_entity, agent_position) in agents.iter() {
                let agent_to_resource_distance = query_item.world_position.0.distance(agent_position.0);
                let resource_to_goal_distance = target.as_vec2().distance(query_item.world_position.0);
                let score = -agent_to_resource_distance - resource_to_goal_distance; // Smaller distance is better
                if score > best_score {
                    best_resource_entity = Some(query_item.entity);
                    best_agent = Some(*agent_entity);
                    best_score = score;
                }
            }
        }

        println!("Best resource entity result: {:?}", best_resource_entity);

        if let Some(resource_entity) = best_resource_entity {
            self.run_time_data = Some(BringResourceRuntimeData {
                concrete_resource_entity: resource_entity,
            });

            resources
                .get_mut(resource_entity)
                .unwrap()
                .reservations
                .0
                .push(Reservation {
                    reserved_by: best_agent.unwrap(),
                    amount: self.item_requirement.amount,
                });

            return best_agent;
        }

        None
    }
}
