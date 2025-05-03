use crate::bundles::rock::Rock;
use crate::bundles::soil::dirt::Dirt;
use crate::bundles::{Id, ItemId, ItemSpawners};
use crate::features::assets::{GltfAssetHandles, GltfAssetId};
use crate::features::map::transition_to_in_game;
use crate::features::misc_components::simple_mesh::SimpleMeshHandles;
use crate::features::misc_components::InWorld;
use crate::features::position::WorldPosition;
use crate::features::seeded_random::RandomSource;
use crate::features::states::AppState;
use bevy::ecs::system::{CachedSystemId, SystemId};
use bevy::gltf::GltfMesh;
use bevy::math::{IVec2, UVec2, Vec2};
use bevy::pbr::NotShadowCaster;
use bevy::prelude::*;
use moonshine_core::save::Save;
use moonshine_view::RegisterView;
use noisy_bevy::simplex_noise_2d_seeded;
use rand::prelude::ThreadRng;
use rand::Rng;
use std::f32::consts::PI;

pub(super) fn map_model_plugin(app: &mut App) {
    app.insert_resource(ReservedCoordinatesHelper::default())
        .insert_resource(FoliageHandles::default())
        .add_systems(
            OnEnter(AppState::MapGeneration),
            (setup_foliage_resources, generate_world, transition_to_in_game).chain(),
        )
        .add_viewable::<MapData>();
}

#[derive(Resource, Debug, Default, Deref, DerefMut)]
pub struct MapSize(pub UVec2);

// A helper resource to store reserved coordinates in map generation
// Used so that we can initialize path finding system only after map
// generation is completely done, for efficiency
#[derive(Resource, Debug, Default, Reflect)]
#[reflect(Resource)]
pub struct ReservedCoordinatesHelper(Vec<IVec2>);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Reflect)]
pub enum TileType {
    Empty,
    Dirt,
    Water,
}

#[derive(Component, Reflect, Default, Clone)]
#[reflect(Component)]
pub struct MapData {
    pub data: Vec<TileType>,
    pub size: UVec2,
}

#[derive(Resource, Debug, Clone, Reflect, Deref)]
pub struct WorldSeed(pub u64);

impl MapData {
    #[expect(unused)]
    pub fn get_tile_type(&self, coordinate: IVec2) -> Option<TileType> {
        let x = (coordinate.x + (self.size.x as i32) / 2) as usize;
        let y = (coordinate.y + (self.size.y as i32) / 2) as usize;
        if x >= self.size.x as usize || y >= self.size.y as usize {
            return None;
        }
        Some(self.data[x + y * self.size.x as usize])
    }

    pub fn convert_to_centered_coordinate(&self, coordinate: UVec2) -> IVec2 {
        let x = (coordinate.x as i32) - ((self.size.x as i32) / 2);
        let y = (coordinate.y as i32) - ((self.size.y as i32) / 2);
        IVec2::new(x, y)
    }

    pub fn world_position_to_top_left_coordinate(&self, coordinate: Vec2) -> UVec2 {
        let x = coordinate.x + (self.size.x as f32) / 2.0;
        let y = coordinate.y + (self.size.y as f32) / 2.0;
        UVec2::new(x as u32, y as u32)
    }

    pub fn center_to_top_left_coordinate(&self, coordinate: IVec2) -> UVec2 {
        let x = coordinate.x + (self.size.x as i32) / 2;
        let y = coordinate.y + (self.size.y as i32) / 2;
        UVec2::new(x as u32, y as u32)
    }

    // Don't be fooled by the fact that this does nothing, currently coordinates just HAPPEN
    // to match global positions, as tile size is exactly 1,1
    pub fn centered_coordinate_to_world_position(&self, coordinate: IVec2) -> Vec2 {
        let x = coordinate.x as f32;
        let y = coordinate.y as f32;
        Vec2::new(x, y)
    }

    pub fn get_tile_type_non_centered(&self, coordinate: UVec2) -> Option<TileType> {
        let x = coordinate.x as usize;
        let y = coordinate.y as usize;
        if x >= self.size.x as usize || y >= self.size.y as usize {
            return None;
        }
        Some(self.data[x + y * self.size.x as usize])
    }

    pub fn set_tile_type(&mut self, coordinate: IVec2, tile_type: TileType) {
        let top_left = self.center_to_top_left_coordinate(coordinate);
        let index = (top_left.y * self.size.x + top_left.x) as usize;

        if index > self.data.len() - 1 {
            panic!(
                "Index out of bounds for set_tile_type {:?}, length of array is: {:?}",
                index,
                self.data.len()
            );
        }
        self.data[index] = tile_type;
    }
}

struct MapGenerationInput {
    current_coordinate: IVec2,
}

fn generate_ground(
    In(MapGenerationInput { current_coordinate }): In<MapGenerationInput>,
    mut map_query: Query<&mut MapData>,
    mut reserved_coordinates: ResMut<ReservedCoordinatesHelper>,
    world_seed: Res<WorldSeed>,
) -> Option<(Dirt, Id, WorldPosition, InWorld)> {
    let mut map_data = map_query.single_mut().expect("Map data not found");

    let map_size = map_data.size;

    let min_bound = map_size.x.min(map_size.y) as f32 - 50.0;

    //let mut dirt_bundles: Vec<(Dirt, Id, WorldPosition, InWorld)> = vec![];

    let mut tile_type = TileType::Dirt;

    const SHORELINE_NOISE_SCALE: f32 = 0.2;
    const SHORELINE_NOISE_THRESHOLD: f32 = 0.5;

    let mapped_value = remap_to_distance_from_center(min_bound, current_coordinate, 0.3, 0.5);
    let noise_value = simplex_noise_2d_seeded(
        current_coordinate.as_vec2() * SHORELINE_NOISE_SCALE,
        world_seed.0 as f32,
    );

    let mut dirt_bundle: Option<(Dirt, Id, WorldPosition, InWorld)> = None;

    if (noise_value / 2.0 + 1.0) * mapped_value > SHORELINE_NOISE_THRESHOLD {
        tile_type = TileType::Water;
        reserved_coordinates.0.push(current_coordinate);
    } else {
        dirt_bundle = Some((
            Dirt,
            Id(ItemId::Dirt),
            WorldPosition(current_coordinate.as_vec2()),
            InWorld,
        ));
    }

    map_data.set_tile_type(current_coordinate, tile_type);

    dirt_bundle
}

pub fn generate_world(world: &mut World) {
    let map_size = UVec2::new(150, 150);
    let map_data = MapData {
        data: vec![TileType::Empty; (map_size.x * map_size.y) as usize],
        size: map_size,
    };

    world.commands().spawn((map_data.clone(), Save));

    let mut dirt_bundles: Vec<(Dirt, Id, WorldPosition, InWorld)> = vec![];
    let mut foliage_bundles: Vec<FoliageBundle> = vec![];
    let mut flower_bundles: Vec<FlowerBundle> = vec![];

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let current_coordinate = map_data.convert_to_centered_coordinate(UVec2::new(x, y));

            let dirt_bundle = world
                .run_system_cached_with(generate_ground, MapGenerationInput { current_coordinate })
                .unwrap();
            if let Some(dirt_bundle) = dirt_bundle {
                dirt_bundles.push(dirt_bundle);
            }

            world
                .run_system_cached_with(generate_rocks, MapGenerationInput { current_coordinate })
                .unwrap();
            world
                .run_system_cached_with(generate_trees, MapGenerationInput { current_coordinate })
                .unwrap();

            let mut foliage_bundle_sum = world
                .run_system_cached_with(generate_foliage, MapGenerationInput { current_coordinate })
                .unwrap();

            foliage_bundles.append(&mut foliage_bundle_sum.foliage_bundles);
            flower_bundles.append(&mut foliage_bundle_sum.flower_bundles);
        }
    }

    world.run_system_cached(generate_test_entities).unwrap();

    world.spawn_batch(dirt_bundles);
    world.spawn_batch(foliage_bundles);
    world.spawn_batch(flower_bundles);

    world.flush();
}

fn remap_to_distance_from_center(
    min_bound: f32,
    centered_coordinate: IVec2,
    start_point_multiplier: f32,
    end_point_multiplier: f32,
) -> f32 {
    let distance_to_center = centered_coordinate.as_vec2().length();
    let shoreline_start_point = min_bound * start_point_multiplier;
    let shoreline_end_point = min_bound * end_point_multiplier;

    if distance_to_center <= shoreline_start_point {
        0.0
    } else if distance_to_center >= shoreline_end_point {
        1.0
    } else {
        (distance_to_center - shoreline_start_point) / (shoreline_end_point - shoreline_start_point)
    }
}

fn generate_rocks(
    In(MapGenerationInput { current_coordinate }): In<MapGenerationInput>,
    mut commands: Commands,
    map_query: Query<&MapData>,
    world_seed: Res<WorldSeed>,
    mut reserved_coordinates: ResMut<ReservedCoordinatesHelper>,
) {
    let map_data = map_query.single().expect("Map data not found");
    let min_bound = map_data.size.x.min(map_data.size.y) as f32;

    let x = current_coordinate.x;
    let y = current_coordinate.y;

    let noise_value =
        simplex_noise_2d_seeded(Vec2::new(x as f32, y as f32) * 0.04, world_seed.0 as f32);

    let mapped_value = remap_to_distance_from_center(min_bound, current_coordinate, 0.4, 0.45);

    let reserved = reserved_coordinates.0.contains(&current_coordinate);

    if (noise_value / 2.0 + 1.0) + mapped_value < 0.65 && !reserved {
        commands.spawn((Rock, InWorld, WorldPosition(current_coordinate.as_vec2())));

        reserved_coordinates.0.push(current_coordinate);
    }
}

fn generate_trees(
    In(MapGenerationInput { current_coordinate }): In<MapGenerationInput>,
    mut commands: Commands,
    world_seed: Res<WorldSeed>,
    mut random_source: ResMut<RandomSource>,
    mut reserved_coordinates: ResMut<ReservedCoordinatesHelper>,
    item_spawners: Res<ItemSpawners>,
) {
    if reserved_coordinates.0.contains(&current_coordinate) {
        return;
    }

    if current_coordinate.x % 2 == 0 || current_coordinate.y % 2 == 0 {
        return;
    }

    const TREE_TYPES: [ItemId; 4] = [
        ItemId::OakTree,
        ItemId::PineTree,
        ItemId::MapleTree,
        ItemId::BarrenTree,
    ];

    let x = current_coordinate.x;
    let y = current_coordinate.y;

    let noise_value = simplex_noise_2d_seeded(
        Vec2::new(x as f32, y as f32) * 0.017,
        world_seed.0 as f32 + 2.0,
    );

    let steepness = 2.2;
    let cutoff = 1.2;
    let spawn_probability = 1.0 / (1.0 + (-steepness * (noise_value - cutoff)).exp());

    if random_source.0.random::<f32>() < spawn_probability {
        let tree_type = TREE_TYPES[rand::rng().random_range(0..TREE_TYPES.len())];
        let item = item_spawners.get(&tree_type).unwrap()(&mut commands);

        commands
            .entity(item)
            .insert((WorldPosition(current_coordinate.as_vec2()), Save, InWorld));

        reserved_coordinates.0.push(current_coordinate);
    }
}

struct EntityGeneration {
    entity_type: ItemId,
    amount: u32,
    func: Option<fn(&mut EntityCommands)>,
}

pub fn generate_test_entities(
    mut commands: Commands,
    map_query: Query<&MapData>,
    mut reserved_coordinates: ResMut<ReservedCoordinatesHelper>,
    item_spawners: Res<ItemSpawners>,
) {
    let map_data = map_query.single().expect("Map data not found");
    let mut rng = rand::rng();

    let test_entities = vec![
        EntityGeneration {
            entity_type: ItemId::Settler,
            amount: 5,
            func: None,
        },
        EntityGeneration {
            entity_type: ItemId::Lumber,
            amount: 300,
            func: None,
        },
        EntityGeneration {
            entity_type: ItemId::PotatoPlantSeed,
            amount: 20,
            func: None,
        },
    ];

    for test_entity in test_entities {
        let mut entity_amount = test_entity.amount;
        let mut max_attempts = 3000;
        while entity_amount > 0 && max_attempts > 0 {
            let x = rng.random_range(0..map_data.size.x);
            let y = rng.random_range(0..map_data.size.y);
            let centered_coordinate = map_data.convert_to_centered_coordinate(UVec2::new(x, y));

            if !reserved_coordinates.0.contains(&centered_coordinate) {
                let item = item_spawners.get(&test_entity.entity_type).unwrap()(&mut commands);
                commands.entity(item).insert((
                    WorldPosition(centered_coordinate.as_vec2()),
                    Save,
                    InWorld,
                ));

                if let Some(func) = test_entity.func {
                    func(&mut commands.entity(item));
                }

                reserved_coordinates.0.push(centered_coordinate);
                entity_amount -= 1;
            }
            max_attempts -= 1;
        }
    }
}

#[derive(Resource, Default)]
struct FoliageHandles {
    grass_material: Option<Handle<StandardMaterial>>,
    grass_blade: Option<Handle<Mesh>>,
    flower_1: Option<Handle<Gltf>>,
    flower_2: Option<Handle<Gltf>>,
}

fn setup_foliage_resources(
    gltf_handles: Res<GltfAssetHandles>,
    gltf_assets: Res<Assets<Gltf>>,
    gltf_meshes: Res<Assets<GltfMesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut foliage_handles: ResMut<FoliageHandles>,
) {
    let gltf_handle = gltf_handles.handles.get(&GltfAssetId::GrassBlade).unwrap();
    let gltf_handle_flower_1 = gltf_handles.handles.get(&GltfAssetId::Flower1).unwrap();
    let gltf_handle_flower_2 = gltf_handles.handles.get(&GltfAssetId::Flower2).unwrap();

    let gltf_asset_grass_blade = gltf_assets.get(gltf_handle).unwrap();

    let mesh_handle = gltf_asset_grass_blade.meshes[0].clone();
    let mesh = gltf_meshes.get(&mesh_handle).unwrap();
    let primitive = mesh.primitives[0].mesh.clone();

    let grass_material = materials.add(StandardMaterial::from_color(Color::srgb(0.6, 0.8, 0.3)));

    *foliage_handles = FoliageHandles {
        grass_material: Some(grass_material),
        grass_blade: Some(primitive),
        flower_1: Some(gltf_handle_flower_1.clone()),
        flower_2: Some(gltf_handle_flower_2.clone()),
    }
}

type FoliageBundle = (Mesh3d, MeshMaterial3d<StandardMaterial>, NotShadowCaster, Transform);
type FlowerBundle = (SceneRoot, Transform);

struct FoliageBundleSum {
    foliage_bundles: Vec<FoliageBundle>,
    flower_bundles: Vec<FlowerBundle>,
}

fn generate_foliage(
    In(MapGenerationInput { current_coordinate }): In<MapGenerationInput>,
    reserved_coordinates: Res<ReservedCoordinatesHelper>,
    foliage_handles: Res<FoliageHandles>,
    gltf_assets: Res<Assets<Gltf>>,
    world_seed: Res<WorldSeed>,
    mut random_source: ResMut<RandomSource>,
) -> FoliageBundleSum {
    if reserved_coordinates.0.contains(&current_coordinate) {
        return FoliageBundleSum {
            foliage_bundles: vec![],
            flower_bundles: vec![],
        };
    }
    
    let gltf_asset_flower_1 = gltf_assets.get(&foliage_handles.flower_1.clone().unwrap()).unwrap();
    let gltf_asset_flower_2 = gltf_assets.get(&foliage_handles.flower_2.clone().unwrap()).unwrap();
    let gltf_asset_flowers = [gltf_asset_flower_1, gltf_asset_flower_2];

    // 80 looks decent but is too heavy especially for in Debug mode
    let max_foliage_amount_per_tile: usize = 1;
    let mut rng = rand::rng();

    let mut foliage_bundles: Vec<FoliageBundle> = vec![];
    let mut flower_bundles: Vec<FlowerBundle> = vec![];

    for _ in 0..max_foliage_amount_per_tile {
        let x = current_coordinate.x;
        let y = current_coordinate.y;

        let noise_value = simplex_noise_2d_seeded(
            Vec2::new(x as f32, y as f32) * 0.02,
            world_seed.0 as f32 + 5.0,
        );

        let steepness = 2.2;
        let cutoff = 1.2;
        let spawn_probability = 1.0 / (1.0 + (-steepness * (noise_value - cutoff)).exp());

        if random_source.0.random::<f32>() < spawn_probability {
            let final_position = current_coordinate.as_vec2()
                + Vec2::new(rng.random_range(-1.0..1.0), rng.random_range(-1.0..1.0));

            let mut transform = Transform::default();
            transform.rotate_y(rng.random_range(0.0..PI * 2.0));
            transform.translation = Vec3::new(final_position.x, 0.0, final_position.y);
            transform.scale = Vec3::splat(rng.random_range(0.6..1.3));

            if rng.random::<f32>() < 0.01 {
                let flower_handle = gltf_asset_flowers.get(rng.random_range(0..2)).unwrap();
                flower_bundles.push((SceneRoot(flower_handle.scenes[0].clone()), transform));
            } else {
                foliage_bundles.push((
                    Mesh3d(foliage_handles.grass_blade.clone().unwrap()),
                    MeshMaterial3d(foliage_handles.grass_material.clone().unwrap()),
                    NotShadowCaster,
                    transform,
                ));
            }
        }
    }

    FoliageBundleSum {
        foliage_bundles,
        flower_bundles,
    }
}
