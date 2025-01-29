use crate::features::misc_components::Prototype;
use crate::features::position::WorldPosition;
use crate::ReflectComponent;
use bevy::app::{App, PostUpdate};
use bevy::asset::AssetServer;
use bevy::core::Name;
use bevy::gltf::GltfAssetLabel;
use bevy::prelude::{Changed, Component, Plugin, Query, Reflect, SceneRoot, Transform, World};
use moonshine_object::{Object, ObjectInstance};
use moonshine_view::{BuildView, RegisterView, ViewCommands, Viewable};

pub struct GltfAssetPlugin;

impl Plugin for GltfAssetPlugin {
    fn build(&self, app: &mut App) {
        // app.add_viewable::<GltfAsset>()
        //     .add_systems(PostUpdate, );
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
#[derive(Debug)]
pub struct GltfAsset(pub String);

impl From<&str> for GltfAsset {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl BuildView for GltfAsset {
    fn build(world: &World, object: Object<GltfAsset>, mut view: ViewCommands<GltfAsset>) {
        if world.get::<Prototype>(object.entity()).is_some() {
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

        println!("Building gltf asset view finished");
    }
}