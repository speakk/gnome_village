use crate::bundles::Prototypes;
use crate::features::assets::GltfAssetHandles;
use crate::features::inventory::{Inventory, InventoryChanged};
use crate::features::misc_components::gltf_asset::{get_scene_from_gltf_data, GltfData};
use crate::features::misc_components::Prototype;
use crate::features::position::WorldPosition;
use crate::ReflectComponent;
use bevy::prelude::*;
use moonshine_object::{Kind, Object, ObjectInstance};
use moonshine_view::{BuildView, RegisterView, ViewCommands, Viewable};

pub struct PreviewCarryPlugin;

impl Plugin for PreviewCarryPlugin {
    fn build(&self, app: &mut App) {
        app.add_viewable::<PreviewCarry>();
        app.add_observer(react_to_inventory_change);
    }
}

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct PreviewCarry;

impl BuildView for PreviewCarry {
    fn build(world: &World, object: Object<Self>, mut view: ViewCommands<Self>) {
        let entity = object.entity();

        if world.get::<Prototype>(entity).is_some() {
            return;
        }

        let inventory = world.get::<Inventory>(entity);

        let Some(inventory) = inventory else {
            return;
        };

        let world_position = world.get::<WorldPosition>(entity).unwrap();
        view.insert(Transform::from_xyz(world_position.x, 0.0, world_position.y));

        let prototypes = world.get_resource::<Prototypes>().unwrap();

        let first_item = inventory.items.keys().next();
        if let Some(first_item) = first_item {
            let prototype = prototypes.0.get(first_item).unwrap();
            let gltf_data = world.get::<GltfData>(*prototype);
            let gltf_assets = world.get_resource::<Assets<Gltf>>().unwrap();
            let gltf_asset_handles = world.get_resource::<GltfAssetHandles>().unwrap();
            if let Some(gltf_data) = gltf_data {
                let scene =
                    match get_scene_from_gltf_data(gltf_asset_handles, gltf_assets, &gltf_data) {
                        Some(value) => value,
                        None => return,
                    };
                view.insert((
                    SceneRoot(scene),
                    Transform::from_xyz(world_position.x, 1.3, world_position.y),
                ));
            }
        }
    }
}

pub fn react_to_inventory_change(
    trigger: Trigger<InventoryChanged>,
    query: Query<(&Inventory, &Viewable<PreviewCarry>, &WorldPosition), With<PreviewCarry>>,
    prototypes: Res<Prototypes>,
    gltf_asset_handles: Res<GltfAssetHandles>,
    gltf_assets: Res<Assets<Gltf>>,
    gltf_data_query: Query<&GltfData>,
    mut commands: Commands,
) {
    let entity = trigger.entity();
    let Ok((inventory, viewable, world_position)) = query.get(entity) else {
        return;
    };

    let first_item = inventory.items.keys().next();
    if let Some(first_item) = first_item {
        let prototype = prototypes.0.get(first_item).unwrap();
        let gltf_data = gltf_data_query.get(*prototype);
        if let Ok(gltf_data) = gltf_data {
            let scene =
                match get_scene_from_gltf_data(&*gltf_asset_handles, &*gltf_assets, &gltf_data) {
                    Some(value) => value,
                    None => return,
                };
            commands.entity(viewable.view().entity()).insert((
                SceneRoot(scene),
                Transform::from_xyz(world_position.x, 1.3, world_position.y),
            ));
        }
    } else {
        commands.entity(viewable.view().entity()).despawn_descendants();
    }
}