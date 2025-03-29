use bevy::color::palettes::basic::NAVY;
use bevy::color::palettes::css::{RED, SIENNA};
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy::render::mesh::CircleMeshBuilder;
use bevy::window::WindowResized;
use std::f32::consts::PI;
use crate::features::sun_light::CurrentTimeOfDay;

pub(super) struct DayCycleIndicatorPlugin;

impl Plugin for DayCycleIndicatorPlugin {
    fn build(&self, app: &mut App) {
        app.init_gizmo_group::<MyGizmos>()
            .add_systems(Startup, setup)
            .init_resource::<CurrentPosition>()
            .add_systems(
                Update,
                (
                    update_current_position,
                    //draw_gizmos,
                    position_indicator,
                    rotate_indicator,
                ),
            );
    }
}

static RADIUS: f32 = 50.0;
static PADDING: f32 = 7.0;

// We can create our own gizmo config group!
#[derive(Default, Reflect, GizmoConfigGroup)]
struct MyGizmos {}

#[derive(Component, Default)]
struct DayCycleCircle;

#[derive(Component, Default)]
struct DayCycleCircleBorder;

#[derive(Resource, Default, Deref)]
struct CurrentPosition(Vec2);

fn setup(
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let texture_handle = asset_server.load("daynight_cycle.png");

    let handle = meshes.add(CircleMeshBuilder::new(RADIUS, 30));
    commands.spawn((
        Mesh2d(handle),
        DayCycleCircle,
        MeshMaterial2d(materials.add(texture_handle)),
    ));

    let handle = meshes.add(CircleMeshBuilder::new(RADIUS + PADDING, 30));
    commands.spawn((
        Mesh2d(handle),
        DayCycleCircleBorder,
        MeshMaterial2d(materials.add(ColorMaterial::from_color(SIENNA))),
    ));
}
//
// fn draw_gizmos(
//     mut gizmos: Gizmos<MyGizmos>,
//     mut resize_reader: EventReader<WindowResized>,
//     mut current_size: Local<Vec2>,
//     current_position: Res<CurrentPosition>
// ) {
//     for event in resize_reader.read() {
//         current_size.x = event.width;
//         current_size.y = event.height;
//         println!("new size: {:?}", current_size);
//     }
//
//     const PADDING: f32 = 5.0;
//
//     gizmos
//         .circle_2d(current_position.0, RADIUS + PADDING, NAVY)
//         .resolution(64);
// }

fn rotate_indicator(
    mut query: Query<&mut Transform, With<DayCycleCircle>>,
    current_time_of_day: Res<CurrentTimeOfDay>
) {
    if current_time_of_day.is_changed() {
        for mut transform in query.iter_mut() {
            transform.rotation = Quat::from_rotation_z(current_time_of_day.time_of_day * PI);
        }
    }
}

fn update_current_position(
    mut resize_reader: EventReader<WindowResized>,
    mut current_position: ResMut<CurrentPosition>,
) {
    for event in resize_reader.read() {
        let new_position = Vec2::new(event.width / 2.0 - RADIUS * 2.0, -event.height / 2.0);
        current_position.0 = new_position;
    }
}

fn position_indicator(
    current_position: Res<CurrentPosition>,
    mut query: Query<
        (
            &mut Transform,
            Option<&DayCycleCircle>,
            Option<&DayCycleCircleBorder>,
        ),
        Or<(With<DayCycleCircle>, With<DayCycleCircleBorder>)>,
    >,
) {
    if current_position.is_changed() {
        for (mut transform, circle, border) in query.iter_mut() {
            let z_index = if circle.is_some() { 1.0 } else { 0.0 };
            transform.translation = current_position.extend(z_index);
        }
    }
}
