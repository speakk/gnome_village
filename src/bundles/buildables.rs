pub mod torch;
pub mod wooden_wall;

use crate::features::health::Health;
use crate::features::inventory::Inventory;
use crate::features::misc_components::ItemAmount;
use bevy::prelude::*;

pub struct BuildablesPlugin;

impl Plugin for BuildablesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BuildableMaterialHandles::default())
            .insert_resource(BluePrintMaterial::default())
            .add_systems(
                Startup,
                (setup_buildable_materials, setup_blueprint_material),
            )
            .add_systems(Update, remove_blueprint_on_inventory_change);
    }
}

#[derive(Resource, Default, Deref)]
pub struct BluePrintMaterial(pub(crate) Option<Handle<StandardMaterial>>);

pub fn setup_blueprint_material(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut blueprint_material: ResMut<BluePrintMaterial>,
) {
    let blueprint_handle = materials.add(Color::srgba(0.3, 0.3, 1.0, 0.4));
    blueprint_material.0 = Some(blueprint_handle);
}

#[derive(Component, Default, Debug, Reflect)]
#[require(Inventory)]
#[reflect(Component)]
pub struct BluePrint;

fn remove_blueprint_on_inventory_change(
    query: Query<(Entity, &Inventory), (With<BluePrint>, Changed<Inventory>)>,
    mut commands: Commands,
) {
    for (entity, inventory) in query.iter() {
        let items_sum = inventory.items.values().sum::<u32>();
        if items_sum > 0 {
            commands.entity(entity).remove::<BluePrint>();
        }
    }
}

#[derive(Resource, Default)]
pub struct BuildableMaterialHandles {
    pub(crate) wood: Option<Handle<StandardMaterial>>,
}

pub fn setup_buildable_materials(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut buildable_material_handles: ResMut<BuildableMaterialHandles>,
) {
    let wood_material = materials.add(Color::srgb(0.6, 0.4, 0.37));
    buildable_material_handles.wood = Some(wood_material);
}

#[derive(Component, Default, Reflect)]
#[require(Health)]
#[reflect(Component)]
pub struct Buildable {
    pub(crate) item_requirements: Vec<ItemAmount>,
    pub construction_progress: f32,
    pub finished: bool,
}

impl Buildable {
    pub fn increase_construction_progress(&mut self, amount: f32) {
        self.construction_progress += amount;

        if self.construction_progress >= 1.0 {
            self.finished = true;
            println!("Buildable finished");
        }
    }
}
