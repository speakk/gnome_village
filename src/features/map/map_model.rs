use crate::bundles::rock::Rock;
use crate::bundles::settler::Settler;
use crate::features::position::WorldPosition;
use bevy::math::{IVec2, UVec2, Vec2};
use bevy::prelude::*;
use bevy::render::render_resource::ShaderType;
use bevy_inspector_egui::egui::{remap, remap_clamp};
use moonshine_core::save::Save;
use noisy_bevy::simplex_noise_2d_seeded;
use rand::Rng;
use std::num::{NonZero, NonZeroU32};

#[derive(Resource, Debug, Default, Deref, DerefMut)]
pub struct MapSize(pub UVec2);

// A helper resource to store reserved coordinates in map generation
// Used so that we can initialize path finding system only after map
// generation is completely done, for efficiency
#[derive(Resource, Debug, Default, Deref, DerefMut)]
pub struct ReservedCoordinatesHelper(Vec<IVec2>);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Reflect)]
pub enum TileType {
    Empty,
    Dirt,
    Water,
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct MapData {
    pub data: Vec<TileType>,
    pub size: UVec2,
}

#[derive(Resource, Debug, Clone, Reflect, Deref)]
pub struct WorldSeed(pub u64);

impl MapData {
    pub fn get_tile_type(&self, coordinate: IVec2) -> Option<TileType> {
        let x = (coordinate.x + (self.size.x as i32) / 2) as usize;
        let y = (coordinate.y + (self.size.y as i32) / 2) as usize;
        if x >= self.size.x as usize || y >= self.size.y as usize {
            return None;
        }
        Some(self.data[x + y * self.size.x as usize])
    }

    pub fn convert_to_centered_coordinate(&self, coordinate: UVec2) -> IVec2 {
        let x = (coordinate.x as i32) - (self.size.x as i32) / 2;
        let y = (coordinate.y as i32) - (self.size.y as i32) / 2;
        IVec2::new(x, y)
    }

    pub fn world_position_to_top_left_coordinate(&self, coordinate: Vec2) -> UVec2 {
        let x = coordinate.x + (self.size.x as f32) / 2.0;
        let y = coordinate.y + (self.size.y as f32) / 2.0;
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
        let index = (self.size.x * (coordinate.x + (self.size.x as i32) / 2) as u32
            + (coordinate.y + (self.size.x as i32) / 2) as u32) as usize;
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

pub fn generate_map_entity(mut commands: Commands, world_seed: Res<WorldSeed>, mut reserved_coordinates: ResMut<ReservedCoordinatesHelper>) {
    let map_size = UVec2::new(150, 150);
    let mut map_data = MapData {
        data: vec![TileType::Empty; (map_size.x * map_size.y) as usize],
        size: map_size,
    };

    let min_bound = map_size.x.min(map_size.y) as f32;

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let mut tile_type = TileType::Dirt;

            const SHORELINE_NOISE_SCALE: f32 = 0.2;
            const SHORELINE_NOISE_THRESHOLD: f32 = 0.5;
            
            let centered_coordinate = map_data.convert_to_centered_coordinate(UVec2::new(x, y));
            let mapped_value = remap_to_distance_from_center(
                min_bound,
                centered_coordinate,
                0.3,
                0.5
            );
            let noise_value = simplex_noise_2d_seeded(centered_coordinate.as_vec2() * SHORELINE_NOISE_SCALE, world_seed.0 as f32);
            
            if (noise_value / 2.0 + 1.0) * mapped_value > SHORELINE_NOISE_THRESHOLD {
                tile_type = TileType::Water;
                reserved_coordinates.0.push(centered_coordinate);
            }

            map_data.set_tile_type(
                IVec2::new(
                    (x as i32) - (map_size.x as i32) / 2,
                    (y as i32) - (map_size.y as i32) / 2,
                ),
                tile_type,
            );
        }
    }

    commands.spawn((map_data, Save));
}

fn remap_to_distance_from_center(min_bound: f32, centered_coordinate: IVec2, start_point_multiplier: f32, end_point_multiplier: f32) -> f32 {
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

pub fn generate_rocks(mut commands: Commands, map_query: Query<&MapData>, world_seed: Res<WorldSeed>, mut reserved_coordinates: ResMut<ReservedCoordinatesHelper>) {
    let map_data = map_query.single();
    let mut rng = rand::thread_rng();

    let min_bound = map_data.size.x.min(map_data.size.y) as f32;
    
    for x in 0..map_data.size.x {
        for y in 0..map_data.size.y {
            let noise_value =
                simplex_noise_2d_seeded(Vec2::new(x as f32, y as f32) * 0.04, world_seed.0 as f32);

            let centered_coordinate = map_data.convert_to_centered_coordinate(UVec2::new(x, y));
            let mapped_value = remap_to_distance_from_center(
                min_bound,
                centered_coordinate,
                0.4,
                0.45
            );

            let reserved = reserved_coordinates.0.contains(&centered_coordinate);

            if (noise_value / 2.0 + 1.0) + mapped_value < 0.65 && !reserved {
                commands.spawn((
                    Rock,
                    WorldPosition(
                        centered_coordinate.as_vec2(),
                    ),
                ));

                reserved_coordinates.0.push(centered_coordinate);
            }
        }
    }
}

pub fn generate_test_entities(mut commands: Commands, map_data_query: Query<&MapData>, mut reserved_coordinates: ResMut<ReservedCoordinatesHelper>) {
    let map_data = map_data_query.single();
    let mut rng = rand::thread_rng();
    let mut settlers_amount = 4;
    let mut max_attempts = 3000;
    while settlers_amount > 0 && max_attempts > 0 {
        let x = rng.gen_range(0..map_data.size.x);
        let y = rng.gen_range(0..map_data.size.y);
        let centered_coordinate = map_data.convert_to_centered_coordinate(UVec2::new(x, y));

        if !reserved_coordinates.0.contains(&centered_coordinate) {
            commands.spawn((Settler, WorldPosition(centered_coordinate.as_vec2()), Save));
            reserved_coordinates.0.push(centered_coordinate);
            settlers_amount -= 1;
        }
        max_attempts -= 1;
    }
}