use crate::bundles::settler::Settler;
use bevy::prelude::*;
use moonshine_view::RegisterView;
use crate::bundles::buildables::BuildablesPlugin;

pub mod rock;
pub mod settler;
pub mod buildables;

pub struct BundlePlugin;

impl Plugin for BundlePlugin {
    fn build(&self, app: &mut App) { 
        app
            .add_plugins(BuildablesPlugin)
            .add_viewable::<Settler>();
    }
}
