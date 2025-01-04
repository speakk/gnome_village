use crate::bundles::settler::Settler;
use bevy::prelude::*;
use moonshine_view::RegisterView;

pub mod settler;

pub struct BundlePlugin;

impl Plugin for BundlePlugin {
    fn build(&self, app: &mut App) {
        app.add_viewable::<Settler>();
    }
}
