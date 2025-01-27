use bevy::prelude::*;
use crate::bundles::clone_entity;
use crate::features::map::map_model::MapData;
use crate::features::misc_components::Prototype;
use crate::features::position::WorldPosition;
use crate::features::user_actions::UserActionState;
use crate::features::world_interaction::mouse_selection::CurrentMouseWorldCoordinate;
use crate::ui::ui_main_actions::build_menu::BuildMenuBuildableSelected;

pub struct BuildActionPlugin;

impl Plugin for BuildActionPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(CurrentBuilding::default())
            .add_systems(Update, follow_mouse_cursor.run_if(in_state(UserActionState::PlacingBuilding)))
            .add_systems(Update, react_to_buildable_menu_selected)
            //.add_systems(Update, ensure_building_preview.run_if(in_state(UserActionState::PlacingBuilding)));
            .add_systems(Update, ensure_building_preview);
    }
}

#[derive(Component, Default)]
pub struct FollowMouseCursor;

#[derive(Resource, Default, Debug, Deref, DerefMut)]
struct CurrentBuilding(Option<Entity>);

fn react_to_buildable_menu_selected(mut build_menu_buildable_selected: EventReader<BuildMenuBuildableSelected>, mut current_building: ResMut<CurrentBuilding>) {
    for event in build_menu_buildable_selected.read() {
        println!("Reacting to buildable menu selected, setting current_building");

        current_building.0 = Some(event.0);
    }
}

//fn ensure_building_preview(mut commands: Commands, mut current_building: ResMut<CurrentBuilding>, world: &mut World) {
fn ensure_building_preview(mut world: &mut World) {
    let current_building = world.resource_mut::<CurrentBuilding>();
    if current_building.is_changed() {
        println!("Current building was changed");
        if let Some(entity) = current_building.0 {
            println!("Current building wasn't empty, cloning entity and inserting follow mouse cursor component and removing prototype");
            let new_entity = clone_entity(world, entity);
            
            let mut commands = world.commands();
            commands.entity(new_entity).insert((Visibility::Visible, FollowMouseCursor)).remove::<Prototype>();
        }
    }
}

pub fn follow_mouse_cursor(mut query: Query<&mut WorldPosition, With<FollowMouseCursor>>,
                           current_mouse_coordinate: Res<CurrentMouseWorldCoordinate>,
                           map_data: Query<&MapData>
) {
    for mut world_position in query.iter_mut() {
        let map_data = map_data.single();
        let location = map_data.centered_coordinate_to_world_position(current_mouse_coordinate.0);
        println!("Following mouse cursor, location: {:?}", location);
        *world_position = WorldPosition(location);
    }
}
// 
// pub fn follow_mouse_cursor(mut query: Query<&mut Transform, With<FollowMouseCursor>>,
//                            current_mouse_coordinate: Res<CurrentMouseWorldCoordinate>,
//                            map_data: Query<&MapData>
// ) {
//     for mut transform in query.iter_mut() {
//         let map_data = map_data.single();
//         let location = map_data.centered_coordinate_to_world_position(current_mouse_coordinate.0);
//         println!("Following mouse cursor, location: {:?}", location);
//         *transform = Transform::from_xyz(location.x, 1.0, location.y);
//     }
// }