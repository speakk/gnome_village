use crate::bundles::Prototypes;
use crate::features::inventory::{Inventory, InventoryChanged};
use crate::features::misc_components::gltf_asset::GltfData;
use crate::ReflectComponent;
use bevy::prelude::*;

pub struct PreviewCarryPlugin;

impl Plugin for PreviewCarryPlugin {
    fn build(&self, app: &mut App) {
        //app.add_observer(react_to_inventory_change);
    }
}

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct PreviewCarry;

#[derive(Component)]
pub struct PreviewCarryEntity;

pub fn react_to_inventory_change(
    trigger: Trigger<InventoryChanged>,
    query: Query<&Inventory, With<PreviewCarry>>,
    children: Query<&Children>,
    prototypes: Res<Prototypes>,
    gltf_data_query: Query<&GltfData>,
    mut commands: Commands,
    preview_carry_entities: Query<&PreviewCarryEntity>,
) {
    let entity = trigger.entity();
    let inventory = query.get(entity);
    
    let Ok(inventory) = inventory else {
        return;
    };

    for child in children.iter_descendants(entity) {
        if preview_carry_entities.get(child).is_ok() {
            commands.entity(child).despawn();
        }
    }

    let first_item = inventory.items.keys().next();
    if let Some(first_item) = first_item {
        let prototype = prototypes.0.get(first_item).unwrap();
        let gltf_data = gltf_data_query.get(*prototype).unwrap();
        commands
            .entity(entity)
            .with_child((GltfData::clone(gltf_data), PreviewCarryEntity, Transform::from_xyz(0.0, 1.0, 0.0)));
    }
}

//pub fn add_preview(query: I)
