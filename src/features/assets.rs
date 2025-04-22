use crate::features::states::AppState::{InGame, Preload};
use bevy::app::App;
use bevy::prelude::*;
use bevy::utils::HashMap;
use std::time::Duration;

pub struct AssetsPlugin;

const SETTLER_PATH: &str = "blender_models/settler.glb";
const TORCH_PATH: &str = "blender_models/wooden_torch.glb";
const OAK_TREE_PATH: &str = "blender_models/plants/oak_tree.glb";
const PINE_TREE_PATH: &str = "blender_models/plants/pine_tree.glb";
const MAPLE_TREE_PATH: &str = "blender_models/plants/maple_tree.glb";
const BARREN_TREE_PATH: &str = "blender_models/plants/bare_tree.glb";
const POTATO_PLANT_PATH: &str = "blender_models/plants/potato_plant.glb";
const POTATO_PATH: &str = "blender_models/plants/potato_harvested.glb";
const LUMBER_PATH: &str = "blender_models/wood.glb";
const WATER_WELL_PATH: &str = "blender_models/well.glb";

#[derive(Debug, Reflect, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GltfAssetId {
    Settler,
    WoodenTorch,
    OakTree,
    PineTree,
    MapleTree,
    BarrenTree,
    Lumber,
    WaterWell,
    PotatoPlant,
    Potato,
}

#[derive(Resource, Default)]
pub struct GltfAssetHandles {
    pub handles: HashMap<GltfAssetId, Handle<Gltf>>,
}

pub enum SettlerAnimationIndices {
    Build,
    Eat,
    Idle,
    Sleep,
    Walk,
}

#[derive(Resource)]
pub struct Animations {
    pub animations: HashMap<GltfAssetId, Vec<AnimationNodeIndex>>,
    pub graph: Handle<AnimationGraph>,
}

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Preload), setup)
            .add_systems(Update, setup_scene_once_loaded.run_if(in_state(InGame)))
            .insert_resource(GltfAssetHandles::default());
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    mut gltf_asset_handles: ResMut<GltfAssetHandles>,
) {
    gltf_asset_handles
        .handles
        .insert(GltfAssetId::Settler, asset_server.load(SETTLER_PATH));
    gltf_asset_handles
        .handles
        .insert(GltfAssetId::WoodenTorch, asset_server.load(TORCH_PATH));
    gltf_asset_handles
        .handles
        .insert(GltfAssetId::OakTree, asset_server.load(OAK_TREE_PATH));
    gltf_asset_handles
        .handles
        .insert(GltfAssetId::PineTree, asset_server.load(PINE_TREE_PATH));
    gltf_asset_handles
        .handles
        .insert(GltfAssetId::MapleTree, asset_server.load(MAPLE_TREE_PATH));
    gltf_asset_handles
        .handles
        .insert(GltfAssetId::BarrenTree, asset_server.load(BARREN_TREE_PATH));
    gltf_asset_handles
        .handles
        .insert(GltfAssetId::Lumber, asset_server.load(LUMBER_PATH));
    gltf_asset_handles
        .handles
        .insert(GltfAssetId::WaterWell, asset_server.load(WATER_WELL_PATH));
    gltf_asset_handles.handles.insert(
        GltfAssetId::PotatoPlant,
        asset_server.load(POTATO_PLANT_PATH),
    );
    gltf_asset_handles
        .handles
        .insert(GltfAssetId::Potato, asset_server.load(POTATO_PATH));

    // Build the animation graph
    let (graph, node_indices) = AnimationGraph::from_clips([
        asset_server.load(GltfAssetLabel::Animation(0).from_asset(SETTLER_PATH)),
        asset_server.load(GltfAssetLabel::Animation(1).from_asset(SETTLER_PATH)),
        asset_server.load(GltfAssetLabel::Animation(2).from_asset(SETTLER_PATH)),
        asset_server.load(GltfAssetLabel::Animation(3).from_asset(SETTLER_PATH)),
        asset_server.load(GltfAssetLabel::Animation(4).from_asset(SETTLER_PATH)),
    ]);

    let mut animations = HashMap::new();
    animations.insert(GltfAssetId::Settler, node_indices);

    // Insert a resource with the current scene information
    let graph_handle = graphs.add(graph);
    commands.insert_resource(Animations {
        animations,
        graph: graph_handle,
    });
}

fn setup_scene_once_loaded(
    mut commands: Commands,
    animations: Res<Animations>,
    mut players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
) {
    for (entity, mut animation_player) in &mut players {
        let mut transitions = AnimationTransitions::new();

        // TODO: Assumes all animations are Settler for now
        transitions
            .play(
                &mut animation_player,
                animations.animations.get(&GltfAssetId::Settler).unwrap()
                    [SettlerAnimationIndices::Idle as usize],
                Duration::ZERO,
            )
            .repeat();

        commands
            .entity(entity)
            .insert(AnimationGraphHandle(animations.graph.clone()))
            .insert(transitions);
    }
}
