use crate::features::movement::{PreviousWorldPosition, WorldPosition};
use bevy::prelude::*;
use moonshine_core::prelude::*;
use moonshine_view::prelude::*;

#[derive(Component, Default)]
pub struct Settler;

#[derive(Bundle, Default)]
pub struct SettlerBundle {
    pub(crate) world_position: WorldPosition,
    pub(crate) previous_world_position: PreviousWorldPosition,
    pub settler: Settler,
}

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
            Transform::from_xyz(transform.x, 1.0, transform.y),
        ));
    }
}
