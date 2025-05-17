use crate::features::misc_components::ItemAmount;
use crate::features::tasks::task::DepositTarget;
use crate::features::tasks::task::RunType;
use crate::features::tasks::task::Task;
use bevy::ecs::entity::MapEntities;
use bevy::prelude::*;
use moonshine_core::prelude::ReflectMapEntities;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
#[reflect(MapEntities)]
pub struct BringResourceRuntimeData {
    pub(crate) concrete_resource_entity: Option<Entity>,
}

impl MapEntities for BringResourceRuntimeData {
    fn map_entities<E: EntityMapper>(&mut self, entity_mapper: &mut E) {
        if let Some(entity) = self.concrete_resource_entity {
            self.concrete_resource_entity = Some(entity_mapper.get_mapped(entity));
        }
    }
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
    #[entities]
    pub run_time_data: BringResourceRuntimeData,
}

impl MapEntities for BringResourceTask {
    fn map_entities<E: EntityMapper>(&mut self, entity_mapper: &mut E) {
        self.target.map_entities(entity_mapper);
        self.run_time_data.map_entities(entity_mapper);
    }
}

impl MapEntities for DepositTarget {
    fn map_entities<E: EntityMapper>(&mut self, entity_mapper: &mut E) {
        if let DepositTarget::Inventory(entity) = self {
            *entity = entity_mapper.get_mapped(*entity);
        }
    }
}
