use bevy::math::{IVec2, UVec2, Vec2};
use bevy::prelude::{Commands, Component, Deref, DerefMut, Resource};
use noisy_bevy::simplex_noise_2d_seeded;

#[derive(Resource, Debug, Default, Deref, DerefMut)]
pub struct MapSize(pub UVec2);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum TileType {
    Empty,
    Dirt,
    Water,
}

#[derive(Component, Default)]
pub struct MapData {
    pub data: Box<[TileType]>,
    pub size: UVec2,
}

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
        let x = ((coordinate.x as i32) - (self.size.x as i32) / 2);
        let y = ((coordinate.y as i32) - (self.size.y as i32) / 2);
        IVec2::new(x, y)
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
        if (index > self.data.len() - 1) {
            panic!(
                "Index out of bounds for set_tile_type {:?}, length of array is: {:?}",
                index,
                self.data.len()
            );
        }
        self.data[index] = tile_type;
    }
}

pub fn generate_map_entity(mut commands: Commands) {
    let map_size = UVec2::new(150, 150);
    let mut map_data = MapData {
        data: vec![TileType::Empty; (map_size.x * map_size.y) as usize].into_boxed_slice(),
        size: map_size,
    };

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_type = TileType::Dirt;
            let seed = 555.0f32;
            let value = simplex_noise_2d_seeded(Vec2::new(x as f32, y as f32), seed);
            map_data.set_tile_type(
                IVec2::new(
                    (x as i32) - (map_size.x as i32) / 2,
                    (y as i32) - (map_size.y as i32) / 2,
                ),
                tile_type,
            );
        }
    }

    commands.spawn((map_data,));
}
