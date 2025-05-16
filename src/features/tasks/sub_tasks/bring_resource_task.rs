use moonshine_core::prelude::ReflectMapEntities;
use crate::features::tasks::task::RunType;
use crate::features::tasks::task::Task;
use bevy::ecs::entity::MapEntities;
use bevy::prelude::*;
use crate::features::misc_components::ItemAmount;
use crate::features::tasks::task::DepositTarget;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, MapEntities)]
pub struct BringResourceRuntimeData {
    #[entities]
    pub(crate) concrete_resource_entity: Entity,
}

#[derive(Component, Debug, Clone, PartialEq, Eq, Hash, Reflect)]
#[require(Task = Task {
    run_type: RunType::Leaf,
    ..Default::default()
})]
#[reflect(Component, MapEntities)]
pub struct BringResourceTask {
    pub item_requirement: ItemAmount,
    #[entities]
    pub target: DepositTarget,
    pub run_time_data: Option<BringResourceRuntimeData>,
}

impl MapEntities for BringResourceTask {
    fn map_entities<E: EntityMapper>(&mut self, entity_mapper: &mut E) {
        self.target.map_entities(entity_mapper);
    }
}

impl MapEntities for DepositTarget {
    fn map_entities<E: EntityMapper>(&mut self, entity_mapper: &mut E) {
        if let DepositTarget::Inventory(entity) = self {
            *entity = entity_mapper.get_mapped(*entity);
        }
    }
}