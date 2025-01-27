use std::borrow::Cow;
use crate::bundles::settler::Settler;
use crate::bundles::{Id, Reservation, Reservations, ResourceItem};
use crate::features::ai::actions::go_to::{GoToAction};
use crate::features::ai::{PathFollow, WorkingOnTask};
use crate::features::map::map_model::MapData;
use crate::features::misc_components::InWorld;
use crate::features::path_finding::{spawn_pathfinding_task, PathFollowFinished, PathFollowResult, PathingGridResource};
use crate::features::position::WorldPosition;
use crate::features::tasks::task::{
    BringResourceData, BringResourceRuntimeData, DepositTarget, Task, TaskType,
};
use beet::prelude::{Action, EndOnRun, LogOnRun, OnRun, SequenceFlow, TargetEntity};
use bevior_tree::node::NodeResult;
use bevior_tree::prelude::{delegate_node, Sequence, TaskBridge};
use bevior_tree::task::{TaskEvent, TaskStatus};
use bevior_tree::{BehaviorTree, BehaviorTreeBundle, TreeStatus};
use bevy::prelude::*;
use crate::features::ai::actions::pick_up::PickUpAction;

pub fn create_bring_resource_tree(
    work_started_query: Query<(&WorkingOnTask, Entity), Added<WorkingOnTask>>,
    item_resources: Query<&WorldPosition, (With<ResourceItem>, With<InWorld>)>,
    tasks: Query<&Task>,
    mut commands: Commands,
) {
    for (working_on_task, worker_entity) in work_started_query.iter() {
        let task = tasks.get(working_on_task.0).unwrap();
        println!("Found WorkingOnTask");

        if let Some(task_type) = &task.task_type {
            println!(
                "Task type: {:?} worked on by agent: {:?}",
                task_type, worker_entity
            );
            if let TaskType::BringResource(bring_resource_data) = task_type {
                println!("Had BringResource task, creating tree");
                let target_coordinate = match bring_resource_data.target {
                    DepositTarget::Coordinate(coordinate) => coordinate,
                    DepositTarget::Inventory(inventory_entity) => panic!(
                        "Inventory target is not supported yet. Inventory entity: {}",
                        inventory_entity
                    ),
                };

                let resource_target = bring_resource_data
                    .run_time_data
                    .unwrap()
                    .concrete_resource_entity;
                let resource_position = item_resources.get(resource_target).unwrap();

                commands
                    .entity(worker_entity)
                    .insert(SequenceFlow)
                    .with_children(|root| {
                        println!("Creating tree, spawning goto");
                        root.spawn((
                            Name::new("GoTo task"),
                            LogOnRun(Cow::from("GoToAction run")),
                            GoToAction {
                                target: resource_position.0.as_ivec2(),
                            },
                            TargetEntity(root.parent_entity()),
                        ));

                        root.spawn((
                            Name::new("PickUp task"),
                            LogOnRun(Cow::from("PickUp task run")),
                            PickUpAction {
                                target_entity: resource_target,
                                amount: bring_resource_data.item_requirement.amount,
                            },
                            TargetEntity(root.parent_entity()),
                        ));

                        root.spawn((
                            Name::new("GoTo task 2"),
                            LogOnRun(Cow::from("GoToAction run 2")),
                            GoToAction {
                                target: target_coordinate,
                            },
                            TargetEntity(root.parent_entity()),
                        ));
                    }).trigger(OnRun);


                // .insert(BehaviorTreeBundle::from_root(Sequence::new(vec![
                //     Box::new(GoTo::new(resource_position.0.as_ivec2())),
                //     Box::new(PickUp::new(
                //         resource_target,
                //         bring_resource_data.item_requirement.amount,
                //     )),
                //     Box::new(GoTo::new(target_coordinate)),
                // ])));
            }
        }
    }
}

#[delegate_node(delegate)]
pub struct DebugPrintTask {
    delegate: TaskBridge,
}

impl DebugPrintTask {
    pub fn new(message: String) -> Self {
        let task = TaskBridge::new(|In(_)| TaskStatus::Complete(NodeResult::Success)).on_event(
            TaskEvent::Enter,
            move |In(_)| {
                println!("Debug print task: {}", message);
            },
        );

        Self { delegate: task }
    }
}

#[delegate_node(delegate)]
pub struct CleanUpTree {
    delegate: TaskBridge,
}

impl CleanUpTree {
    pub fn new() -> Self {
        let task = TaskBridge::new(|In(_)| TaskStatus::Running).on_event(
            TaskEvent::Enter,
            |In(worker_entity), mut commands: Commands| {
                println!("Cleaning up tree!");
                commands
                    .entity(worker_entity)
                    .remove::<WorkingOnTask>()
                    .remove::<BehaviorTree>()
                    .remove::<TreeStatus>();
            },
        );

        Self { delegate: task }
    }
}

pub fn score_bring_resource(
    resources_query: &mut Query<
        (Entity, &WorldPosition, &Id, &mut Reservations),
        (With<ResourceItem>, With<InWorld>),
    >,
    agents: &Vec<(Entity, &WorldPosition)>,
    bring_resource_data: &mut BringResourceData,
    _others_query: &Query<(Entity, &WorldPosition), (Without<ResourceItem>, Without<Settler>)>,
) -> Option<Entity> {
    println!("Scoring bring resource");
    let mut best_resource_entity: Option<Entity> = None;
    let mut best_agent: Option<Entity> = None;
    let mut best_score = -9999.0;

    let target = match bring_resource_data.target {
        DepositTarget::Coordinate(coordinate) => coordinate,
        DepositTarget::Inventory(inventory_entity) => panic!(
            "Inventory target is not supported yet. Inventory entity: {}",
            inventory_entity
        ),
    };

    let valid_resources = resources_query
        .iter()
        .filter(|(_, _, id, reservations)| {
            let reserved_amount = reservations.0.iter().map(|r| r.amount).sum::<u32>();
            id.0 == bring_resource_data.item_requirement.item_id
                && reserved_amount < bring_resource_data.item_requirement.amount
        })
        .collect::<Vec<_>>();

    for (resource_entity, resource_position, _id, reservations) in valid_resources.iter() {
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
