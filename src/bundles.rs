use crate::bundles::buildables::BuildablesPlugin;
use crate::bundles::settler::Settler;
use crate::features::misc_components::simple_mesh::MiscComponentsPlugin;
use crate::features::misc_components::Prototype;
use crate::utils::entity_clone::CloneEntityCommandsExt;
use bevy::prelude::*;
use bevy::utils::HashMap;
use moonshine_core::save::Save;
use moonshine_view::RegisterView;
use std::string::ToString;

pub mod buildables;
pub mod rock;
pub mod settler;

pub struct BundlePlugin;

impl Plugin for BundlePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Prototypes(HashMap::new()))
            .insert_resource(ItemSpawners(HashMap::new()))
            .add_plugins(MiscComponentsPlugin)
            .add_plugins(BuildablesPlugin)
            .add_viewable::<Settler>();
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

struct ConstructionCost {
    amount: u32,
    requirement: Vec<ItemId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct BuildableData {
    construction_costs: i32,
    name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ItemId {
    WoodenWall = 0,
    Rock = 1,
    Settler = 2,
    WoodenTorch = 3,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id(ItemId);

#[derive(Resource, Deref, DerefMut)]
pub struct ItemSpawners(pub(crate) HashMap<ItemId, fn(&mut Commands) -> Entity>);

// Entities which have metadata that is required before an entity
// is actually created in-game can be added here (for example anything that shows up in menus)
#[derive(Resource)]
pub struct Prototypes(pub(crate) HashMap<ItemId, Entity>);

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

// pub trait ItemCreator {
//     fn create_item(&mut self, bundle_type: &ItemId) -> Entity;
// }
//
// impl<'w, 's> ItemCreator for Commands<'w, 's> {
//     fn create_item(&mut self, bundle_type: &ItemId) -> Entity {
//         match bundle_type {
//             ItemId::WoodenWall => self.spawn((WoodenWall,)).id(),
//             ItemId::Rock => self.spawn((Rock,)).id(),
//             ItemId::Settler => self.spawn((Settler,)).id(),
//             ItemId::WoodenTorch => self.spawn((WoodenTorch,)).id(),
//         }
//     }
// }
