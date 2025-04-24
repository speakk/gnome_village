use bevy::color::palettes::css::RED;
use crate::features::misc_components::simple_mesh::{
    SimpleMesh, SimpleMeshHandles, SimpleMeshType,
};
use crate::features::position::WorldPosition;
use crate::ReflectComponent;
use bevy::pbr::NotShadowReceiver;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use moonshine_object::{Object, ObjectInstance};
use moonshine_view::prelude::*;
use crate::features::map::map_model::MapData;

pub fn destruct_target_plugin(app: &mut App) {
    // app.init_resource::<DestructTargetMaterial>();
    // app.add_systems(Startup, create_destruct_target_material);
    // app.add_viewable::<DestructTarget>();
    app.add_systems(Update, draw_destruct_target_gizmo);
}

fn draw_destruct_target_gizmo(
    query: Query<&WorldPosition, With<DestructTarget>>,
    mut gizmos: Gizmos,
) {
    for world_position in query.iter() {
        let isometry = Isometry3d::from_translation(Vec3::new(world_position.x + 0.3, 0.0, world_position.y + 0.3));
        gizmos.rect(isometry, Vec2::splat(0.2), Color::srgb(0.9, 0.4, 0.35));
        gizmos.rect(isometry, Vec2::splat(0.1), Color::srgb(0.9, 0.4, 0.35));
    }
}

//
// fn create_destruct_target_material(
//     mut destruct_target_material: ResMut<DestructTargetMaterial>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     let mut material = StandardMaterial::from_color(Color::srgb(1.0, 0.0, 0.0));
//     destruct_target_material.0 = Some(materials.add(material));
// }
//
// #[derive(Resource, Default)]
// struct DestructTargetMaterial(Option<Handle<StandardMaterial>>);

#[derive(Component, Debug, Clone, Default, Reflect)]
#[reflect(Component)]
pub struct DestructTarget;
//
// impl BuildView for DestructTarget {
//     fn build(
//         world: &World,
//         object: Object<DestructTarget>,
//         mut view: ViewCommands<DestructTarget>,
//     ) {
//         let mesh_handles = world.get_resource::<SimpleMeshHandles>().unwrap();
//         let mesh_handle = mesh_handles.0.get(&SimpleMeshType::Plane).unwrap();
//         let material = world.get_resource::<DestructTargetMaterial>().unwrap();
//         let transform = world.get::<WorldPosition>(object.entity()).unwrap();
//
//         view.insert((
//             Mesh3d(mesh_handle.clone()),
//             MeshMaterial3d(material.0.clone().unwrap()),
//             Transform::from_xyz(transform.x + 0.5, 1.0, transform.y + 0.5).with_scale(Vec3::splat(0.15)),
//             ));
//     }
// }
