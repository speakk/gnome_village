pub mod torch;
pub mod wooden_wall;

use crate::bundles::buildables::torch::WoodenTorch;
use crate::bundles::buildables::wooden_wall::{on_add_blueprint, on_remove_blueprint, view_wall_moved, WallPlugin, WoodenWall};
use crate::features::map::map_view::{MapMeshHandles, MeshType};
use crate::features::misc_components::Prototype;
use crate::features::states::AppState;
use bevy::prelude::*;
use moonshine_view::prelude::*;
use crate::bundles::{ItemId, ItemSpawners, Prototypes};

pub struct BuildablesPlugin;

impl Plugin for BuildablesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BuildableMaterialHandles::default())
            .insert_resource(BuildableMeshHandles::default())
            .insert_resource(BluePrintMaterial::default())
            .insert_resource(BuildableBundleTypes::default())
            .add_plugins(WallPlugin)
            .add_systems(Startup, (setup_buildable_materials, setup_buildable_meshes, setup_blueprint_material))
            .add_systems(OnEnter(AppState::InGame), add_buildable_prototypes);
    }
}

#[derive(Resource, Default, Deref)]
pub struct BluePrintMaterial(Option<Handle<StandardMaterial>>);

pub fn setup_blueprint_material(mut materials: ResMut<Assets<StandardMaterial>>, mut blueprint_material: ResMut<BluePrintMaterial>) {
    let blueprint_handle = materials.add(Color::srgba(0.3, 0.3, 1.0, 0.4));
    blueprint_material.0 = Some(blueprint_handle);
}

#[derive(Component, Default)]
pub struct BluePrint;

#[derive(Resource, Default)]
pub struct BuildableMaterialHandles {
    wood: Option<Handle<StandardMaterial>>,
}

#[derive(Resource, Default)]
pub struct BuildableMeshHandles {
    wall: Option<Handle<Mesh>>,
}

pub fn setup_buildable_materials(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut buildable_material_handles: ResMut<BuildableMaterialHandles>,
) {
    let wood_material = materials.add(Color::srgb(0.6, 0.4, 0.37));
    buildable_material_handles.wood = Some(wood_material);
}

pub fn setup_buildable_meshes(
    mut buildable_mesh_handles: ResMut<BuildableMeshHandles>,
    map_mesh_handles: Res<MapMeshHandles>,
) {
    // Reuse map mesh cuboid for efficiency
    buildable_mesh_handles.wall = map_mesh_handles.get(&MeshType::Cuboid).cloned();
}

// macro_rules! apply_prototype_commands {
//     ( $y:expr,$( $x:expr ),* ) => {
//         {
//             $(
//                 $y.spawn(($x, Prototype, Visibility::Hidden));
//             )*
//         }
//     };
// }

pub fn add_buildable_prototypes(mut commands: Commands,
                                mut item_spawners: ResMut<ItemSpawners>,
                                mut prototypes: ResMut<Prototypes>,
) {
    //apply_prototype_commands!(commands, WoodenWall, WoodenTorch);
    
    prototypes.0.insert(ItemId::WoodenTorch, commands.spawn((WoodenTorch,Prototype)).id());
    prototypes.0.insert(ItemId::WoodenWall, commands.spawn((WoodenWall,Prototype)).id());
    
    item_spawners.0.insert(ItemId::WoodenTorch, |commands| {
        commands.spawn((WoodenTorch,)).id()
    });
    
    item_spawners.0.insert(ItemId::WoodenWall, |commands| {
        commands.spawn((WoodenWall,)).id()
    });
}

pub fn uhh(mut commands: Commands, item_spawners: Res<ItemSpawners>) {
    item_spawners.0.get(&ItemId::Rock).unwrap()(&mut commands);
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Buildable;

#[derive(Resource)]
pub struct BuildableBundleTypes(pub Vec<ItemId>);

impl Default for BuildableBundleTypes {
    fn default() -> Self {
        Self(vec![
            ItemId::WoodenWall,
            ItemId::WoodenTorch,
        ])
    }
}