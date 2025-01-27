use crate::features::position::WorldPosition;
use bevy::prelude::*;
use moonshine_object::{Object, ObjectInstance};
use moonshine_view::{BuildView, ViewCommands};

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct InWorld;

// Explicitly not-in-game, for use in menus etc
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Prototype;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct LightSource {
    pub intensity: f32,
    pub color: Color,
}

impl BuildView for LightSource {
    fn build(world: &World, object: Object<LightSource>, mut view: ViewCommands<LightSource>) {
        if world.get::<InWorld>(object.entity()).is_none() {
            return;
        }

        let transform = world.get::<WorldPosition>(object.entity()).unwrap();

        let light_source = world.get::<LightSource>(object.entity()).unwrap();

        view.insert((
            SpotLight {
                color: light_source.color,
                intensity: light_source.intensity,
                ..default()
            },
            Transform::from_xyz(transform.x, 0.0, transform.y),
        ));
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct GltfAsset(pub String);

impl From<&str> for GltfAsset {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl BuildView for GltfAsset {
    fn build(world: &World, object: Object<GltfAsset>, mut view: ViewCommands<GltfAsset>) {
        if world.get::<InWorld>(object.entity()).is_none() {
            return;
        }

        let transform = world.get::<WorldPosition>(object.entity()).unwrap();
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        let gltf_asset = world.get::<GltfAsset>(object.entity()).unwrap();

        view.insert((
            SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(gltf_asset.0.clone()))),
            Transform::from_xyz(transform.x, 0.0, transform.y),
            Name::new("Gltf asset view"),
        ));
    }
}
