use crate::bundles::buildables::BuildablesPlugin;
use bevy::prelude::*;
use bevy::utils::HashMap;

pub mod buildables;
pub mod plants;
pub mod resources;
pub mod rock;
pub mod settler;
pub mod soil;
pub mod spawners;
pub mod category_tags;

pub struct BundlePlugin;

impl Plugin for BundlePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Prototypes(HashMap::new()))
            .insert_resource(ItemSpawners(HashMap::new()))
            .add_systems(Update, react_to_emptied_stack)
            .add_plugins(BuildablesPlugin);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum ItemId {
    WoodenWall = 0,
    Rock = 1,
    Settler = 2,
    WoodenTorch = 3,
    Lumber = 4,
    OakTree = 5,
    Water = 6,
    Dirt = 7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum ItemCategory {
    Tree = 0
}

#[derive(Resource)]
pub struct ItemCategories(HashMap<ItemCategory, Vec<ItemId>>);

pub fn setup_item_categories(mut item_categories: ResMut<ItemCategories>) {
    item_categories.0.insert(ItemCategory::Tree, vec![ItemId::OakTree]);
}

#[derive(Component, Reflect, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[require(ItemStack, Reservations)]
#[reflect(Component)]
pub struct ResourceItem;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
#[reflect(Component)]
pub struct ItemStack(pub u32);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Reflect)]
pub struct Reservation {
    pub reserved_by: Entity,
    pub amount: u32,
}

#[derive(Component, Default, Debug, Clone, PartialEq, Eq, Hash, Reflect)]
#[reflect(Component)]
pub struct Reservations(pub Vec<Reservation>);

impl Default for ItemStack {
    fn default() -> Self {
        ItemStack(1)
    }
}

pub fn react_to_emptied_stack(
    query: Query<(Entity, &ItemStack), Changed<ItemStack>>,
    mut commands: Commands,
) {
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
