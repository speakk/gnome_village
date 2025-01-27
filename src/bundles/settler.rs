use crate::features::position::{PreviousWorldPosition, WorldPosition};
use bevy::prelude::*;
use crate::bundles::{ItemId, Id};
use moonshine_core::prelude::*;
use moonshine_view::prelude::*;

#[derive(Component, Default, Reflect)]
#[require(Id(|| Id(ItemId::Settler)), WorldPosition, PreviousWorldPosition, Name(|| "Settler"))]
#[reflect(Component)]
pub struct Settler;

impl BuildView for Settler {
    fn build(world: &World, object: Object<Settler>, mut view: ViewCommands<Settler>) {
        println!("Building view for settler");
        let transform = world.get::<WorldPosition>(object.entity()).unwrap();
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        view.insert((
            SceneRoot(
                asset_server
                    .load(GltfAssetLabel::Scene(0).from_asset("blender_models/settler.glb")),
            ),
            Transform::from_xyz(transform.x, 0.0, transform.y),
            Name::new("Settler view"),
        ));
    }
}
