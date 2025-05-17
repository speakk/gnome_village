use crate::bundles::rock::Rock;
use crate::bundles::soil::dirt::Dirt;
use crate::bundles::{Id, ItemId, Prototypes};
use crate::features::assets::{GltfAssetHandles, GltfAssetId};
use crate::features::misc_components::{InWorld, Prototype};
use crate::features::position::WorldPosition;
use crate::features::seeded_random::RandomSource;
use crate::features::states::AppState;
use crate::ui::colours::THEME_1_800;
use crate::ui::FONT_BOLD;
use bevy::ecs::system::{CachedSystemId, SystemId};
use bevy::gltf::GltfMesh;
use bevy::math::{IVec2, UVec2, Vec2};
use bevy::pbr::NotShadowCaster;
use bevy::prelude::*;
use bevy::render::view::NoFrustumCulling;
use moonshine_core::save::Save;
use moonshine_view::RegisterView;
use noisy_bevy::simplex_noise_2d_seeded;
use rand::prelude::ThreadRng;
use rand::Rng;
use std::f32::consts::PI;

pub(super) fn map_model_plugin(app: &mut App) {
    app.insert_resource(ReservedCoordinatesHelper::default())
        .insert_resource(FoliageHandles::default())
        .insert_resource(GenerateWorldTimer::default())
        .add_systems(OnEnter(AppState::MapGeneration), (create_loading_ui,))
        .add_systems(
            Update,
            generate_world.run_if(in_state(AppState::MapGeneration)),
        )
        .add_viewable::<MapData>();
}

pub fn create_loading_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("Creating loading UI");
    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::from(crate::ui::colours::THEME_1_200),
            ..Default::default()
        },
        IsDefaultUiCamera,
        StateScoped(AppState::MapGeneration),
        Msaa::Off,
    ));

    let bold_font_handle = asset_server.load(FONT_BOLD);

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        StateScoped(AppState::MapGeneration),
        children![(
            Text::new("Generating world!".to_uppercase()),
            TextFont {
                font: bold_font_handle,
                font_size: 84.0,
                ..default()
            },
            TextColor(THEME_1_800),
        )],
    ));
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
    DirtGrassyLight,
    DirtGrassyFull,
    DirtGrassyHalf,
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
    map_size: UVec2,
}

// TODO: Absolutely ridiculous, using this so that UI has a chance to actually show before world
// generation kicks in.
#[derive(Resource, Debug, Reflect)]
struct GenerateWorldTimer(Timer);

impl Default for GenerateWorldTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.2, TimerMode::Once))
    }
}

pub fn generate_world(world: &mut World) {
    world.resource_scope(|world: &mut World, mut timer: Mut<GenerateWorldTimer>| {
        let time = world.get_resource::<Time>().unwrap();
        if timer.0.tick(time.delta()).just_finished() {
            println!("Generating world!");
            let map_size = UVec2::new(150, 150);
            let map_data = MapData {
                data: vec![TileType::Empty; (map_size.x * map_size.y) as usize],
                size: map_size,
            };

            world.commands().spawn((map_data.clone(), Save));

            let mut dirt_bundles: Vec<DirtBundle> = vec![];
            let mut rock_bundles: Vec<RockBundle> = vec![];
            let mut foliage_bundles: Vec<FoliageBundle> = vec![];
            let mut flower_bundles: Vec<FlowerBundle> = vec![];

            let mut reserved_coordinates: Vec<IVec2> = vec![];

            for x in 0..map_size.x {
                for y in 0..map_size.y {
                    let current_coordinate =
                        map_data.convert_to_centered_coordinate(UVec2::new(x, y));

                    let generate_ground_result: GenerateGroundResult = world
                        .run_system_cached_with(
                            generate_ground,
                            MapGenerationInput {
                                current_coordinate,
                                map_size,
                            },
                        )
                        .unwrap();

                    if let Some(dirt_bundle) = generate_ground_result.dirt_bundle {
                        dirt_bundles.push(dirt_bundle);
                    }

                    let mut had_any_reserved = false;
                    
                    let ground_reserved = generate_ground_result.coordinate_reserved;
                    if !ground_reserved {
                        let generate_rock_result: GenerateRockResult = world
                            .run_system_cached_with(
                                generate_rocks,
                                MapGenerationInput {
                                    current_coordinate,
                                    map_size,
                                },
                            )
                            .unwrap();

                        if let Some(bundle) = generate_rock_result.rock_bundle {
                            rock_bundles.push(bundle);
                        }

                        if !generate_rock_result.reserved {
                            world
                                .run_system_cached_with(
                                    generate_trees,
                                    MapGenerationInput {
                                        current_coordinate,
                                        map_size,
                                    },
                                )
                                .unwrap();
                        } else {
                            had_any_reserved = true;
                        }
                    } else {
                        had_any_reserved = true;
                    }
                    
                    if had_any_reserved {
                        reserved_coordinates.push(current_coordinate);
                    }

                    // let mut foliage_bundle_sum = world
                    //     .run_system_cached_with(
                    //         generate_foliage,
                    //         MapGenerationInput { current_coordinate },
                    //     )
                    //     .unwrap();
                    //
                    // foliage_bundles.append(&mut foliage_bundle_sum.foliage_bundles);
                    // flower_bundles.append(&mut foliage_bundle_sum.flower_bundles);
                }
            }

            world
                .run_system_cached_with(generate_test_entities, reserved_coordinates)
                .unwrap();

            world.spawn_batch(rock_bundles);
            world.spawn_batch(dirt_bundles);
            world.spawn_batch(foliage_bundles);
            world.spawn_batch(flower_bundles);

            //world.run_system_cached(create_foliage_mesh).unwrap();

            world.flush();

            let next_state = world.get_resource_mut::<NextState<AppState>>();
            next_state.unwrap().set(AppState::InGame);
            println!("Finished generating world!");
        }
    });
}

// fn create_foliage_mesh(mut commands: Commands, foliage_handles: Res<FoliageHandles>) {
//     let handle = foliage_handles.grass_blade.clone().unwrap();
//     // TODO: Probably stick this into View
//     commands.spawn((
//         Mesh3d(handle),
//         crate::features::map::foliage_instancing::InstanceMaterialData(
//             (1..=10)
//                 .flat_map(|x| (1..=10).map(move |y| (x as f32 / 10.0, y as f32 / 10.0)))
//                 .map(|(x, y)| crate::features::map::foliage_instancing::InstanceData {
//                     position: Vec3::new(x * 10.0 - 5.0, y * 10.0 - 5.0, 0.0),
//                     scale: 1.0,
//                     color: LinearRgba::from(Color::hsla(x * 360., y, 0.5, 1.0)).to_f32_array(),
//                 })
//                 .collect(),
//         ),
//         NoFrustumCulling,
//     ));
// }

type DirtBundle = (Dirt, Id, WorldPosition, InWorld);

struct GenerateGroundResult {
    dirt_bundle: Option<DirtBundle>,
    coordinate_reserved: bool,
}

fn generate_ground(
    In(MapGenerationInput {
        current_coordinate,
        map_size,
    }): In<MapGenerationInput>,
    mut map_data: Query<&mut MapData>,
    world_seed: Res<WorldSeed>,
) -> GenerateGroundResult {
    let mut map_data = map_data.single_mut().unwrap();
    let min_bound = map_size.x.min(map_size.y) as f32 - 50.0;

    let mut tile_type = TileType::Dirt;

    const SHORELINE_NOISE_SCALE: f32 = 0.2;
    const SHORELINE_NOISE_THRESHOLD: f32 = 0.5;

    let mapped_value = remap_to_distance_from_center(min_bound, current_coordinate, 0.3, 0.5);
    let noise_value = simplex_noise_2d_seeded(
        current_coordinate.as_vec2() * SHORELINE_NOISE_SCALE,
        world_seed.0 as f32,
    );

    let mut dirt_bundle: Option<(Dirt, Id, WorldPosition, InWorld)> = None;

    let mut reserved = false;

    if (noise_value / 2.0 + 1.0) * mapped_value > SHORELINE_NOISE_THRESHOLD {
        tile_type = TileType::Water;
        reserved = true;
    } else {
        dirt_bundle = Some((
            Dirt,
            Id(ItemId::Dirt),
            WorldPosition(current_coordinate.as_vec2()),
            InWorld,
        ));

        let noise_value = simplex_noise_2d_seeded(
            current_coordinate.as_vec2() * 0.03,
            world_seed.0 as f32 + 0.1,
        );

        if noise_value > -0.6 {
            tile_type = TileType::DirtGrassyLight;
        }

        if noise_value > 0.3 {
            tile_type = TileType::DirtGrassyHalf;
        }

        if noise_value > 0.7 {
            tile_type = TileType::DirtGrassyFull;
        }
    }

    map_data.set_tile_type(current_coordinate, tile_type);

    GenerateGroundResult {
        dirt_bundle,
        coordinate_reserved: reserved,
    }
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

type RockBundle = (Rock, InWorld, WorldPosition);

struct GenerateRockResult {
    reserved: bool,
    rock_bundle: Option<RockBundle>,
}

fn generate_rocks(
    In(MapGenerationInput {
        current_coordinate,
        map_size,
    }): In<MapGenerationInput>,
    world_seed: Res<WorldSeed>,
) -> GenerateRockResult {
    let min_bound = map_size.x.min(map_size.y) as f32;

    let x = current_coordinate.x;
    let y = current_coordinate.y;

    let noise_value =
        simplex_noise_2d_seeded(Vec2::new(x as f32, y as f32) * 0.04, world_seed.0 as f32);

    let mapped_value = remap_to_distance_from_center(min_bound, current_coordinate, 0.4, 0.45);

    if (noise_value / 2.0 + 1.0) + mapped_value < 0.65 {
        GenerateRockResult {
            rock_bundle: Some((Rock, InWorld, WorldPosition(current_coordinate.as_vec2()))),
            reserved: true,
        }
    } else {
        GenerateRockResult {
            rock_bundle: None,
            reserved: false,
        }
    }
}

fn generate_trees(
    In(MapGenerationInput {
        current_coordinate,
        map_size: _map_size,
    }): In<MapGenerationInput>,
    mut commands: Commands,
    world_seed: Res<WorldSeed>,
    mut random_source: ResMut<RandomSource>,
    mut reserved_coordinates: ResMut<ReservedCoordinatesHelper>,
    prototypes: Res<Prototypes>,
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
        commands
            .entity(*prototypes.0.get(&tree_type).unwrap())
            .clone_and_spawn()
            .insert((WorldPosition(current_coordinate.as_vec2()), Save, InWorld))
            .remove::<Prototype>();

        reserved_coordinates.0.push(current_coordinate);
    }
}

struct EntityGeneration {
    entity_type: ItemId,
    amount: u32,
    func: Option<fn(&mut EntityCommands)>,
}

pub fn generate_test_entities(
    In(reserved_coordinates): In<Vec<IVec2>>,
    mut commands: Commands,
    map_query: Query<&MapData>,
    prototypes: Res<Prototypes>,
) {
    let map_data = map_query.single().expect("Map data not found");
    let mut rng = rand::rng();

    let test_entities = vec![
        EntityGeneration {
            entity_type: ItemId::Settler,
            amount: 1,
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

    let mut reserved_coordinates = reserved_coordinates.clone();

    for test_entity in test_entities {
        let mut entity_amount = test_entity.amount;
        let mut max_attempts = 3000;
        while entity_amount > 0 && max_attempts > 0 {
            let x = rng.random_range(0..map_data.size.x);
            let y = rng.random_range(0..map_data.size.y);
            let centered_coordinate = map_data.convert_to_centered_coordinate(UVec2::new(x, y));

            if !reserved_coordinates.contains(&centered_coordinate) {
                let item = commands
                    .entity(*prototypes.0.get(&test_entity.entity_type).unwrap())
                    .clone_and_spawn()
                    .insert((WorldPosition(centered_coordinate.as_vec2()), Save, InWorld))
                    .remove::<Prototype>()
                    .id();

                if let Some(func) = test_entity.func {
                    func(&mut commands.entity(item));
                }

                reserved_coordinates.push(centered_coordinate);
                entity_amount -= 1;
            }
            max_attempts -= 1;
        }
    }
}

#[derive(Resource, Default)]
pub struct FoliageHandles {
    grass_material: Option<Handle<StandardMaterial>>,
    grass_blade: Option<Handle<Mesh>>,
    flower_1: Option<Handle<Gltf>>,
    flower_2: Option<Handle<Gltf>>,
}

pub fn setup_foliage_resources(
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

type FoliageBundle = (
    Mesh3d,
    MeshMaterial3d<StandardMaterial>,
    NotShadowCaster,
    Transform,
);
type FlowerBundle = (SceneRoot, Transform);

struct FoliageBundleSum {
    foliage_bundles: Vec<FoliageBundle>,
    flower_bundles: Vec<FlowerBundle>,
}

fn generate_foliage(
    In(MapGenerationInput {
        current_coordinate,
        map_size: _map_size,
    }): In<MapGenerationInput>,
    reserved_coordinates: Res<ReservedCoordinatesHelper>,
    foliage_handles: Res<FoliageHandles>,
    gltf_assets: Res<Assets<Gltf>>,
    map_query: Query<&MapData>,
    mut random_source: ResMut<RandomSource>,
) -> FoliageBundleSum {
    if reserved_coordinates.0.contains(&current_coordinate) {
        return FoliageBundleSum {
            foliage_bundles: vec![],
            flower_bundles: vec![],
        };
    }

    let map_data = map_query.single().expect("Map data not found");

    let gltf_asset_flower_1 = gltf_assets
        .get(&foliage_handles.flower_1.clone().unwrap())
        .unwrap();
    let gltf_asset_flower_2 = gltf_assets
        .get(&foliage_handles.flower_2.clone().unwrap())
        .unwrap();
    let gltf_asset_flowers = [gltf_asset_flower_1, gltf_asset_flower_2];

    // 80 looks decent but is too heavy especially for in Debug mode
    let max_foliage_amount_per_tile: usize = 1;
    let mut rng = rand::rng();

    let mut foliage_bundles: Vec<FoliageBundle> = vec![];
    let mut flower_bundles: Vec<FlowerBundle> = vec![];

    for _ in 0..max_foliage_amount_per_tile {
        let x = current_coordinate.x;
        let y = current_coordinate.y;
        //
        // let noise_value = simplex_noise_2d_seeded(
        //     Vec2::new(x as f32, y as f32) * 0.02,
        //     world_seed.0 as f32 + 5.0,
        // );

        let steepness = 2.2;
        let cutoff = 1.2;
        let tile_type_multiplier = match map_data.get_tile_type(current_coordinate) {
            Some(TileType::DirtGrassyFull) => 0.3,
            Some(TileType::DirtGrassyHalf) => 0.15,
            Some(TileType::DirtGrassyLight) => 0.06,
            _ => 0.01,
        };

        //let spawn_probability = 1.0 / (1.0 + (-steepness * (noise_value - cutoff)).exp()) * tile_type_multilier;
        let spawn_probability = tile_type_multiplier;

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
