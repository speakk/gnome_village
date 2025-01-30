use std::time::Duration;
use crate::features::states::AppState::Preload;
use bevy::app::App;
use bevy::prelude::*;
use bevy::utils::HashMap;

pub struct AssetsPlugin;

const SETTLER_PATH: &str = "blender_models/settler.glb";

#[derive(Debug, Reflect, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnimationId {
    Settler
}

// Build, eat, idle, sleep, walk

pub enum SettlerAnimationIndices {
    Build,
    Eat,
    Idle,
    Sleep,
    Walk,
}

#[derive(Resource)]
pub struct Animations {
    pub animations: HashMap<AnimationId, Vec<AnimationNodeIndex>>,
    pub graph: Handle<AnimationGraph>,
}

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Preload), setup)
            .add_systems(Update, setup_scene_once_loaded);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    // Build the animation graph
    let (graph, node_indices) = AnimationGraph::from_clips([
        asset_server.load(GltfAssetLabel::Animation(0).from_asset(SETTLER_PATH)),
        asset_server.load(GltfAssetLabel::Animation(1).from_asset(SETTLER_PATH)),
        asset_server.load(GltfAssetLabel::Animation(2).from_asset(SETTLER_PATH)),
        asset_server.load(GltfAssetLabel::Animation(3).from_asset(SETTLER_PATH)),
        asset_server.load(GltfAssetLabel::Animation(4).from_asset(SETTLER_PATH)),
    ]);
    
    let mut animations = HashMap::new();
    animations.insert(AnimationId::Settler, node_indices);

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
                animations.animations.get(&AnimationId::Settler).unwrap()[SettlerAnimationIndices::Idle as usize],
                Duration::ZERO,
            )
            .repeat();
        
        commands
            .entity(entity)
            .insert(AnimationGraphHandle(animations.graph.clone()))
            .insert(transitions);
    }
}
