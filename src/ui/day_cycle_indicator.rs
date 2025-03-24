use bevy::color::palettes::basic::NAVY;
use bevy::color::palettes::css::RED;
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy::render::mesh::CircleMeshBuilder;
use bevy::window::WindowResized;

pub(super) struct DayCycleIndicatorPlugin;

impl Plugin for DayCycleIndicatorPlugin {
    fn build(&self, app: &mut App) {
        app.init_gizmo_group::<MyGizmos>()
            .add_systems(Startup, setup)
            .add_systems(Update, draw_gizmos);
    }
}

// We can create our own gizmo config group!
#[derive(Default, Reflect, GizmoConfigGroup)]
struct MyGizmos {}

#[derive(Component, Default)]
struct DayCycleCircle;

fn setup(
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
    //mut materials: ResMut<Assets<ColorMaterial>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let texture_handle = asset_server.load("daynight_cycle.png");
    
    let handle = meshes.add(CircleMeshBuilder::new(20.0, 30));
    commands.spawn((
        Mesh2d(handle),
        DayCycleCircle,
        MeshMaterial2d(materials.add(texture_handle)),
        //MeshMaterial2d(materials.add(ColorMaterial::from_color(RED))),
    ));
}

fn draw_gizmos(
    mut gizmos: Gizmos<MyGizmos>,
    mut resize_reader: EventReader<WindowResized>,
    mut current_size: Local<Vec2>,
    mut commands: Commands,
    mut query: Query<(Entity, Option<&mut Transform>), With<DayCycleCircle>>,
) {
    for event in resize_reader.read() {
        current_size.x = event.width;
        current_size.y = event.height;
        println!("new size: {:?}", current_size);
    }

    let radius = 20.0;
    let gizmo_position = Vec2::new(
        current_size.x / 2.0 - radius * 2.0,
        -current_size.y / 2.0 + radius * 2.0,
    );
    // 
    // gizmos
    //     .circle_2d(gizmo_position, radius, NAVY)
    //     .resolution(64);

    for (entity, transform) in query.iter_mut() {
        if let Some(mut transform) = transform {
            transform.translation = gizmo_position.extend(0.0);
        }
    }
}

