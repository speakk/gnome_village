use crate::bundles::settler::Settler;
use crate::features::ai::{PathFollow, WorkingOnTask};
use crate::features::assets::{AnimationId, Animations, SettlerAnimationIndices};
use crate::features::misc_components::Prototype;
use crate::features::position::WorldPosition;
use crate::ReflectComponent;
use bevy::app::{App, PostUpdate};
use bevy::asset::AssetServer;
use bevy::core::Name;
use bevy::gltf::GltfAssetLabel;
use bevy::prelude::*;
use moonshine_object::{Object, ObjectInstance};
use moonshine_view::{BuildView, RegisterView, ViewCommands, Viewable};
use std::time::Duration;
use crate::features::ai::actions::build::IsBuilding;

pub struct GltfAssetPlugin;

impl Plugin for GltfAssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_animation, react_to_path_follow, react_to_path_idle, react_to_build, react_to_work_finished),
        )
        .add_viewable::<GltfAsset>();
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
#[derive(Debug)]
pub struct GltfAsset(pub String);

#[derive(Component, Reflect)]
#[reflect(Component)]
#[derive(Debug)]
pub struct GltfAnimation {
    pub animation_id: AnimationId,
    pub animation_indices: Vec<usize>,
    pub current_animation_index: usize,
    pub should_play: bool,
}

impl From<&str> for GltfAsset {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl BuildView for GltfAsset {
    fn build(world: &World, object: Object<GltfAsset>, mut view: ViewCommands<GltfAsset>) {
        if world.get::<Prototype>(object.entity()).is_some() {
            return;
        }

        let transform = world.get::<WorldPosition>(object.entity()).unwrap();
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        let gltf_asset = world.get::<GltfAsset>(object.entity()).unwrap();

        view.insert((
            SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(gltf_asset.0.clone()))),
            Transform::from_xyz(transform.x, 0.0, transform.y),
            Name::new("Gltf asset view"),
        ));

        println!("Building gltf asset view finished");
    }
}

fn update_animation(
    query: Query<(&GltfAnimation, &Viewable<GltfAsset>), Changed<GltfAnimation>>,
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

fn react_to_work_finished(mut removed: RemovedComponents<WorkingOnTask>, mut gltf_animations: Query<&mut GltfAnimation>) {
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
