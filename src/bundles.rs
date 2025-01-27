use crate::bundles::buildables::wooden_wall::WoodenWall;
use crate::bundles::buildables::BuildablesPlugin;
use crate::bundles::settler::Settler;
use crate::features::misc_components::Prototype;
use crate::utils::entity_clone::CloneEntityCommandsExt;
use bevy::prelude::*;
use moonshine_core::save::Save;
use moonshine_view::RegisterView;
use crate::bundles::buildables::torch::WoodenTorch;
use crate::bundles::rock::Rock;

pub mod buildables;
pub mod rock;
pub mod settler;

pub struct BundlePlugin;

impl Plugin for BundlePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BuildablesPlugin).add_viewable::<Settler>();
    }
}

pub fn make_concrete_from_prototype(prototype: Entity, mut commands: Commands) -> Entity {
    let cloned = commands.clone_entity(prototype);
    commands
        .entity(cloned)
        .insert(Save)
        .remove::<Prototype>()
        .id()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum BundleType {
    WoodenWall = 0,
    Rock = 1,
    Settler = 2,
    WoodenTorch = 3,
}

/*
trait MyTraitExt {
  fn spawn_my_thing(&mut self) -> &mut EntityCommands;
}

impl MyTraitExt for Commands {
  fn spawn_my_thing(&mut self) -> &mut EntityCommands {
    self.spawn(..)
  }
}
 */

trait BundleGenerator {
    fn generate(&mut self, bundle_type: &BundleType) -> Entity;
}

impl<'w, 's> BundleGenerator for Commands<'w, 's> {
    fn generate(&mut self, bundle_type: &BundleType) -> Entity {
        match bundle_type {
            BundleType::WoodenWall => self.spawn((WoodenWall,)).id(),
            BundleType::Rock => self.spawn((Rock,)).id(),
            BundleType::Settler => self.spawn((Settler,)).id(),
            BundleType::WoodenTorch => self.spawn((WoodenTorch,)).id(),
        }
    }
}
//
// impl BundleType {
//     fn create(&self) -> Entity {
//         match self { BundleType::WoodenWall => {
//
//         } }
//     }
// }
