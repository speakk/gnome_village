use crate::features::movement::{PhysicalTranslation, PreviousPhysicalTranslation};
use bevy::prelude::*;
use moonshine_core::prelude::*;
use moonshine_view::prelude::*;

#[derive(Component, Default)]
pub struct Settler;

#[derive(Bundle, Default)]
pub struct SettlerBundle {
    pub(crate) physical_translation: PhysicalTranslation,
    pub(crate) previous_physical_translation: PreviousPhysicalTranslation,
    pub settler: Settler,
}

impl BuildView for Settler {
    fn build(world: &World, object: Object<Settler>, mut view: ViewCommands<Settler>) {
        println!("Building view for settler");
        let transform = world.get::<PhysicalTranslation>(object.entity()).unwrap();
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
