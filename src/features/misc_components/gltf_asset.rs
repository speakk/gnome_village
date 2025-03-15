use crate::features::ai::actions::build::IsBuilding;
use crate::features::ai::{PathFollow, WorkingOnTask};
use crate::features::assets::{Animations, GltfAssetHandles, GltfAssetId, SettlerAnimationIndices};
use crate::features::inventory::InInventory;
use crate::features::misc_components::{InWorld, Prototype};
use crate::features::position::WorldPosition;
use crate::ReflectComponent;
use bevy::app::App;
use bevy::core::Name;
use bevy::prelude::*;
use moonshine_object::{Kind, Object, ObjectInstance};
use moonshine_view::{BuildView, RegisterView, ViewCommands, Viewable};
use std::time::Duration;

pub struct GltfAssetPlugin;

impl Plugin for GltfAssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_animation,
                update_scene,
                react_to_path_follow,
                react_to_path_idle,
                react_to_build,
                react_to_work_finished,
            ),
        )
        .add_view::<GltfData, GltfValid>();
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
#[derive(Debug, Clone)]
pub struct GltfData {
    pub asset_id: GltfAssetId,
    pub scene_name: Option<String>,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
#[derive(Debug)]
pub struct GltfAnimation {
    pub animation_id: GltfAssetId,
    pub animation_indices: Vec<usize>,
    pub current_animation_index: usize,
    pub should_play: bool,
}

struct GltfValid;

impl Kind for GltfValid {
    type Filter = (Without<Prototype>, Without<InInventory>);
}

impl BuildView<GltfData> for GltfValid {
    fn build(world: &World, object: Object<GltfData>, mut view: ViewCommands<GltfData>) {
        let gltf_data = world.get::<GltfData>(object.entity()).unwrap();
        let gltf_assets = world.get_resource::<Assets<Gltf>>().unwrap();
        let gltf_asset_handles = world.get_resource::<GltfAssetHandles>().unwrap();

        let scene = match get_scene_from_gltf_data(gltf_asset_handles, gltf_assets, &gltf_data) {
            Some(value) => value,
            None => return,
        };

        let world_position = world.get::<WorldPosition>(object.entity()).unwrap();
        view.insert((
            SceneRoot(scene),
            Transform::from_xyz(world_position.x, 0.0, world_position.y),
            Name::new("Gltf asset view"),
        ));

        println!("Building gltf asset view finished");
    }
}

pub fn get_scene_from_gltf_data(
    asset_handles: &GltfAssetHandles,
    gltf_assets: &Assets<Gltf>,
    gltf_data: &&GltfData,
) -> Option<Handle<Scene>> {
    let asset_handle = asset_handles
        .handles
        .get(&gltf_data.asset_id)
        .expect("Could not find asset handle");

    let gltf = gltf_assets.get(asset_handle)?;

    let scene = get_scene_handle(gltf_data, gltf);
    Some(scene)
}

fn update_animation(
    query: Query<(&GltfAnimation, &Viewable<GltfData>), Changed<GltfAnimation>>,
    children_query: Query<&Children>,
    mut animation_player: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
    animations: Res<Animations>,
) {
    for (gltf_animation, viewable) in query.iter() {
        if gltf_animation.should_play {
            let view_entity = viewable.view().entity();
            for descendant in children_query.iter_descendants(view_entity) {
                if let Ok((mut animation_player, mut transitions)) =
                    animation_player.get_mut(descendant)
                {
                    let animations = animations
                        .animations
                        .get(&gltf_animation.animation_id)
                        .unwrap();
                    transitions
                        .play(
                            &mut animation_player,
                            animations[gltf_animation.current_animation_index],
                            Duration::ZERO,
                        )
                        .repeat();
                }
            }
        }
    }
}

fn update_scene(
    query: Query<(&GltfData, &Viewable<GltfData>), (Changed<GltfData>, With<InWorld>)>,
    gltf_assets: Res<Assets<Gltf>>,
    gltf_asset_handles: Res<GltfAssetHandles>,
    mut commands: Commands,
) {
    for (gltf_data, viewable) in query.iter() {
        let asset_handle = gltf_asset_handles
            .handles
            .get(&gltf_data.asset_id)
            .expect("Could not find asset handle");

        let gltf = gltf_assets.get(asset_handle).unwrap();

        let scene = get_scene_handle(&gltf_data, gltf);

        let view_entity = viewable.view().entity();

        commands
            .entity(view_entity)
            .despawn_descendants()
            .insert(SceneRoot(scene));
    }
}

fn get_scene_handle(gltf_data: &&GltfData, gltf: &Gltf) -> Handle<Scene> {
    if let Some(scene_name) = &gltf_data.scene_name {
        gltf.named_scenes[scene_name.as_str()].clone()
    } else {
        gltf.scenes[0].clone()
    }
}

fn react_to_path_follow(mut query: Query<&mut GltfAnimation, Added<PathFollow>>) {
    for mut gltf_animation in query.iter_mut() {
        gltf_animation.should_play = true;
        gltf_animation.current_animation_index = SettlerAnimationIndices::Walk as usize;
    }
}

fn react_to_build(mut query: Query<&mut GltfAnimation, Added<IsBuilding>>) {
    for mut gltf_animation in query.iter_mut() {
        gltf_animation.should_play = true;
        gltf_animation.current_animation_index = SettlerAnimationIndices::Build as usize;
    }
}

fn react_to_work_finished(
    mut removed: RemovedComponents<WorkingOnTask>,
    mut gltf_animations: Query<&mut GltfAnimation>,
) {
    for entity in removed.read() {
        if let Ok(mut gltf_animation) = gltf_animations.get_mut(entity) {
            gltf_animation.current_animation_index = SettlerAnimationIndices::Idle as usize;
        }
    }
}

fn react_to_path_idle(
    mut param_set: ParamSet<(
        Query<&mut GltfAnimation, (Added<GltfAnimation>, Without<WorkingOnTask>)>,
        Query<&mut GltfAnimation>,
    )>,
    mut removed: RemovedComponents<WorkingOnTask>,
) {
    {
        for mut gltf_animation in param_set.p0().iter_mut() {
            gltf_animation.should_play = true;
            gltf_animation.current_animation_index = SettlerAnimationIndices::Idle as usize;
        }
    }

    for entity in removed.read() {
        if let Ok(mut gltf_animation) = param_set.p1().get_mut(entity) {
            gltf_animation.should_play = true;
            gltf_animation.current_animation_index = 2;
        }
    }
}
