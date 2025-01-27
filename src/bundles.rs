use crate::bundles::buildables::BuildablesPlugin;
use crate::bundles::settler::Settler;
use crate::features::misc_components::MiscComponentsPlugin;
use crate::features::misc_components::Prototype;
use crate::utils::entity_clone::CloneEntityCommandsExt;
use bevy::prelude::*;
use bevy::utils::HashMap;
use moonshine_core::save::Save;
use moonshine_view::RegisterView;
use crate::bundles::spawners::setup_spawners;

pub mod buildables;
pub mod resources;
pub mod rock;
pub mod settler;
pub mod spawners;

pub struct BundlePlugin;

impl Plugin for BundlePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Prototypes(HashMap::new()))
            .insert_resource(ItemSpawners(HashMap::new()))
            .add_plugins(MiscComponentsPlugin)
            .add_systems(Update, react_to_emptied_stack)
            .add_plugins(BuildablesPlugin);
    }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum ItemId {
    WoodenWall = 0,
    Rock = 1,
    Settler = 2,
    WoodenTorch = 3,
    Lumber = 4,
}

#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[require(ItemStack, Reservations)]
pub struct ResourceItem;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct ItemStack(pub u32);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Reflect)]
pub struct Reservation {
    pub reserved_by: Entity,
    pub amount: u32,
}

#[derive(Component, Default, Debug, Clone, PartialEq, Eq, Hash, Reflect)]
pub struct Reservations(pub Vec<Reservation>);

impl Default for ItemStack {
    fn default() -> Self {
        ItemStack(1)
    }
}

pub fn react_to_emptied_stack(query: Query<(Entity, &ItemStack), Changed<ItemStack>>, mut commands: Commands) {
    for (entity, item_stack) in query.iter() {
        if item_stack.0 == 0 {
            commands.entity(entity).despawn();
        }
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut)]
pub struct Id(pub(crate) ItemId);

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
