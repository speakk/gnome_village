use bevy::asset::AssetContainer;
use crate::bundles::Prototypes;
use crate::features::inventory::{Inventory, InventoryChanged};
use crate::features::misc_components::gltf_asset::{get_scene_from_gltf_data, GltfData};
use crate::ReflectComponent;
use bevy::prelude::*;
use moonshine_object::{Kind, Object, ObjectInstance};
use moonshine_view::{BuildView, ViewCommands, Viewable};
use crate::features::assets::GltfAssetHandles;
use crate::features::position::WorldPosition;

pub struct PreviewCarryPlugin;

impl Plugin for PreviewCarryPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(react_to_inventory_change);
    }
}

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct PreviewCarry;

impl BuildView for PreviewCarry {
    fn build(world: &World, object: Object<Self>, mut view: ViewCommands<Self>) {
        let entity = object.entity();
        let inventory = world.get::<Inventory>(entity);

        let Some(inventory) = inventory else {
            return;
        };
        
        let world_position = world.get::<WorldPosition>(entity).unwrap();
        
        let prototypes = world.get_resource::<Prototypes>().unwrap();

        let first_item = inventory.items.keys().next();
        if let Some(first_item) = first_item {
            let prototype = prototypes.0.get(first_item).unwrap();
            let gltf_data = world.get::<GltfData>(*prototype);
            let gltf_assets = world.get_resource::<Assets<Gltf>>().unwrap();
            let gltf_asset_handles = world.get_resource::<GltfAssetHandles>().unwrap();
            if let Some(gltf_data) = gltf_data {
                let scene = match get_scene_from_gltf_data(gltf_asset_handles, gltf_assets, &gltf_data) {
                    Some(value) => value,
                    None => return,
                };
                view.insert((SceneRoot(scene), Transform::from_xyz(world_position.x, 1.0, world_position.y)));
            }
        }
    }
}

pub fn react_to_inventory_change(
    trigger: Trigger<InventoryChanged>,
    query: Query<(&Inventory, &Viewable<PreviewCarry>), With<PreviewCarry>>,
    prototypes: Res<Prototypes>,
    gltf_asset_handles: Res<GltfAssetHandles>,
    gltf_assets: Res<Assets<Gltf>>,
    gltf_data_query: Query<&GltfData>,
    mut commands: Commands,
) {
    let entity = trigger.entity();
    let Ok((inventory, viewable)) = query.get(entity) else {
        return;
    };
    // 
    // for child in children.iter_descendants(entity) {
    //     if preview_carry_entities.get(child).is_ok() {
    //         commands.entity(child).despawn();
    //     }
    // }

    let first_item = inventory.items.keys().next();
    if let Some(first_item) = first_item {
        let prototype = prototypes.0.get(first_item).unwrap();
        let gltf_data = gltf_data_query.get(*prototype);
        if let Ok(gltf_data) = gltf_data {
            let scene = match get_scene_from_gltf_data(&*gltf_asset_handles, &*gltf_assets, &gltf_data) {
                Some(value) => value,
                None => return,
            };
            commands.entity(viewable.view().entity()).insert(SceneRoot(scene));
        }
    }
}

//pub fn add_preview(query: I)
