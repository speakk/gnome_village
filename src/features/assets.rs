use bevy::app::{App, Startup};
use bevy::prelude::*;
use crate::features::states::AppState::Preload;

pub struct AssetsPlugin;

const SETTLER_PATH: &str = "blender_models/settler.glb";

#[derive(Resource)]
pub struct Animations {
    animations: Vec<AnimationNodeIndex>,
    graph: Handle<AnimationGraph>,
}


impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Preload), setup);
    }
}

fn setup(mut commands: Commands,
asset_server: Res<AssetServer>,
mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    // Build the animation graph
    let (graph, node_indices) = AnimationGraph::from_clips([
        asset_server.load(GltfAssetLabel::Animation(2).from_asset(SETTLER_PATH)),
        asset_server.load(GltfAssetLabel::Animation(1).from_asset(SETTLER_PATH)),
        asset_server.load(GltfAssetLabel::Animation(0).from_asset(SETTLER_PATH)),
    ]);

    // Insert a resource with the current scene information
    let graph_handle = graphs.add(graph);
    commands.insert_resource(Animations {
        animations: node_indices,
        graph: graph_handle,
    });
}