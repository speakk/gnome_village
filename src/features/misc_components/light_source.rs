use crate::ReflectComponent;
use bevy::color::Color;
use bevy::prelude::{default, Component, Reflect, Transform, World};
use moonshine_view::{BuildView, ViewCommands};
use moonshine_object::{Object, ObjectInstance};
use bevy::pbr::PointLight;
use crate::features::misc_components::Prototype;
use crate::features::position::WorldPosition;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct LightSource {
    pub intensity: f32,
    pub color: Color,
}

impl Default for LightSource {
    fn default() -> Self {
        Self {
            intensity: 100000.0,
            color: Color::WHITE,
        }
    }
}

impl BuildView for LightSource {
    fn build(world: &World, object: Object<LightSource>, mut view: ViewCommands<LightSource>) {
        if world.get::<Prototype>(object.entity()).is_some() {
            return;
        }
        println!("Building light source");

        let transform = world.get::<WorldPosition>(object.entity()).unwrap();
        let light_source = world.get::<LightSource>(object.entity()).unwrap();

        view.insert((
            PointLight {
                color: light_source.color,
                intensity: light_source.intensity,
                range: 2.0,
                shadows_enabled: true,
                ..default()
            },
            Transform::from_xyz(transform.x, 1.5, transform.y),
        ));
    }
}