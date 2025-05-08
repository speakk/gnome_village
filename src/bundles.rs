use crate::bundles::buildables::BuildablesPlugin;
use bevy::prelude::*;
use bevy_platform::collections::HashMap;
use crate::bundles::buildables::wooden_wall::WoodenWall;

pub mod buildables;
pub mod category_tags;
pub mod plants;
pub mod resources;
pub mod rock;
pub mod settler;
pub mod soil;
pub mod prototypes;

pub struct BundlePlugin;

impl Plugin for BundlePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Prototypes(HashMap::new()))
            .insert_resource(ItemCategories(HashMap::new()))
            .add_systems(Startup, setup_item_categories)
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
    WaterWell = 8,
    PotatoPlant = 9,
    PotatoPlantSeed = 10,
    Potato = 11,
    Nitrogen = 12,
    Potassium = 13,
    PineTree = 14,
    MapleTree = 15,
    BarrenTree = 16,
}
// 
// #[derive(Clone, Reflect, Debug, PartialEq, Eq, Hash)]
// pub enum ItemIds {
//     WoodenWall(crate::bundles::buildables::wooden_wall::WoodenWall)
// }
// 
// #[derive(Component, Reflect)]
// pub struct Dropsies {
//     pub drops: Vec<ItemIds>
// }
// 
// #[derive(Component, Reflect)]
// #[require(Dropsies = Dropsies { drops: vec![ItemIds::WoodenWall(WoodenWall)] })]
// pub struct OakkiTree;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum ItemCategory {
    Tree = 0,
    Rocks = 1,
}

#[derive(Resource)]
pub struct ItemCategories(pub(crate) HashMap<ItemCategory, Vec<ItemId>>);

pub fn setup_item_categories(mut item_categories: ResMut<ItemCategories>) {
    item_categories.0.insert(
        ItemCategory::Tree,
        vec![
            ItemId::OakTree,
            ItemId::PineTree,
            ItemId::MapleTree,
            ItemId::BarrenTree,
        ],
    );
    item_categories
        .0
        .insert(ItemCategory::Rocks, vec![ItemId::Rock]);
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
    // let ItemIds::WoodenWall(wooden_wall) = ItemIds::WoodenWall(WoodenWall);
    // commands.spawn(wooden_wall.clone());
    // 
    for (entity, item_stack) in query.iter() {
        if item_stack.0 == 0 {
            commands.entity(entity).despawn();
        }
    }
}

#[derive(Component, Reflect, Debug, Clone, Copy, PartialEq, Eq, Hash, Deref, DerefMut)]
#[reflect(Component)]
pub struct Id(pub(crate) ItemId);

// Entities which have metadata that is required before an entity
// is actually created in-game can be added here (for example anything that shows up in menus)
#[derive(Resource)]
pub struct Prototypes(pub(crate) HashMap<ItemId, Entity>);