use crate::features::misc_components::gltf_asset::GltfData;
use crate::features::misc_components::ItemAmount;
use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy::prelude::{Component, Reflect};
use rand::Rng;

pub struct PlantsPlugin;

impl Plugin for PlantsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlantStageAdvanced>()
            .add_systems(Update, initialize_plant)
            .add_systems(FixedUpdate, update_growth_process)
            .add_observer(update_gltf_based_on_growth_stage);
    }
}

#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct Plant {
    pub growth_stages: usize,
    pub current_growth_stage: u8,
    pub growth_speed: f32,
    pub growth_requirements: Vec<ItemAmount>,
    pub current_stage_growth_process: f32,
    pub finished_growing: bool,
    pub random_growth_multiplier: f32,
}

#[derive(Event)]
pub struct PlantStageAdvanced;

#[derive(Event)]
pub struct PlantFinishedGrowing;

impl Default for Plant {
    fn default() -> Self {
        Self {
            growth_stages: 1,
            current_growth_stage: 0,
            growth_speed: 1.0,
            growth_requirements: vec![],
            current_stage_growth_process: 0.0,
            finished_growing: false,
            random_growth_multiplier: rand::rng().random_range(0.6..1.2),
        }
    }
}

pub fn initialize_plant(mut commands: Commands, query: Query<Entity, Added<Plant>>) {
    for entity in query.iter() {
        commands.entity(entity).trigger(PlantStageAdvanced);
    }
}

pub fn update_growth_process(
    mut query: Query<(Entity, &mut Plant)>,
    time: Res<Time<Fixed>>,
    mut commands: Commands,
) {
    for (entity, mut plant) in query.iter_mut() {
        if plant.finished_growing {
            continue;
        }

        plant.current_stage_growth_process +=
            plant.growth_speed * plant.random_growth_multiplier * time.delta_secs();
        if plant.current_stage_growth_process >= 1.0 {
            plant.current_stage_growth_process = 0.0;
            plant.current_growth_stage += 1;

            commands.entity(entity).trigger(PlantStageAdvanced);

            if plant.current_growth_stage >= plant.growth_stages as u8 - 1 {
                plant.finished_growing = true;
                commands.entity(entity).trigger(PlantFinishedGrowing);
            }
        }
    }
}

pub fn update_gltf_based_on_growth_stage(
    trigger: Trigger<PlantStageAdvanced>,
    mut query: Query<(&Plant, &mut GltfData)>,
) {
    if let Ok((plant, mut gltf_data)) = query.get_mut(trigger.entity()) {
        let current_scene_name = format!("stage_{}", plant.current_growth_stage);
        if gltf_data.scene_name != Some(current_scene_name.clone()) {
            gltf_data.scene_name = Some(current_scene_name.clone());
        }
    }
}
