use crate::bundles::spawners::setup_spawners_and_prototypes;
use crate::features::map::map_model::setup_foliage_resources;
use crate::features::map::map_view::create_map_materials;
use crate::features::states::AppState;
use crate::features::states::AppState::{InGame, LoadAssets, Preload};
use bevy::app::App;
use bevy::prelude::*;
use bevy_platform::collections::HashMap;
use std::time::Duration;
use bevy::asset::{AssetPath, LoadState, UntypedAssetId};
use bevy_asset_loader::prelude::*;

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
const GRASS_BLADE: &str = "blender_models/foliage/grass_blade.glb";
const FLOWER_1: &str = "blender_models/foliage/flower_1.glb";
const FLOWER_2: &str = "blender_models/foliage/flower_2.glb";

#[derive(Debug, Reflect, Copy, Clone, PartialEq, Eq, Hash)]
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
    GrassBlade,
    Flower1,
    Flower2,
    Unknown,
}

// #[derive(Resource, Default)]
// pub struct GltfAssetHandles {
//     pub handles: HashMap<GltfAssetId, Handle<Gltf>>,
// }

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
        //app.insert_resource(GltfAssetHandles::default())
        app.add_systems(
                OnEnter(Preload),
                (
                    setup,
                    create_map_materials,
                    setup_foliage_resources,
                    setup_spawners_and_prototypes,
                    |mut next_state: ResMut<NextState<AppState>>| {
                        next_state.set(AppState::MapGeneration);
                    },
                )
                    .chain(),
            )
            .add_systems(Update, setup_scene_once_loaded.run_if(in_state(InGame)));

        app.add_loading_state(LoadingState::new(LoadAssets).continue_to_state(Preload).load_collection::<GltfAssetHandles>());
    }
}

#[derive(AssetCollection, Resource)]
pub struct GltfAssetHandles {
    #[asset(
        paths(
            "blender_models/settler.glb",
            "blender_models/wooden_torch.glb",
            "blender_models/plants/oak_tree.glb",
            "blender_models/plants/pine_tree.glb",
            "blender_models/plants/maple_tree.glb",
            "blender_models/plants/bare_tree.glb",
            "blender_models/plants/potato_plant.glb",
            "blender_models/plants/potato_harvested.glb",
            "blender_models/wood.glb",
            "blender_models/well.glb",
            "blender_models/foliage/grass_blade.glb",
            "blender_models/foliage/flower_1.glb",
            "blender_models/foliage/flower_2.glb"
        ),
        collection(typed, mapped)
    )]
    pub handles: HashMap<GltfAssetId, Handle<Gltf>>,
}

impl MapKey for GltfAssetId {
    fn from_asset_path(path: &AssetPath) -> Self {
        let stem = path
            .path()
            .file_stem()
            .unwrap()
            .to_str()
            .expect("Path should be valid UTF-8")
            .to_string();
        match stem.as_str() {
            "settler" => GltfAssetId::Settler,
            "wooden_torch" => GltfAssetId::WoodenTorch,
            "oak_tree" => GltfAssetId::OakTree,
            "pine_tree" => GltfAssetId::PineTree,
            "maple_tree" => GltfAssetId::MapleTree,
            "bare_tree" => GltfAssetId::BarrenTree,
            "potato_plant" => GltfAssetId::PotatoPlant,
            "potato_harvested" => GltfAssetId::Potato,
            "wood" => GltfAssetId::Lumber,
            "well" => GltfAssetId::WaterWell,
            "grass_blade" => GltfAssetId::GrassBlade,
            "flower_1" => GltfAssetId::Flower1,
            "flower_2" => GltfAssetId::Flower2,
            n => GltfAssetId::Unknown,
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    // gltf_asset_handles
    //     .handles
    //     .insert(GltfAssetId::Settler, asset_server.load(SETTLER_PATH));
    // gltf_asset_handles
    //     .handles
    //     .insert(GltfAssetId::WoodenTorch, asset_server.load(TORCH_PATH));
    // gltf_asset_handles
    //     .handles
    //     .insert(GltfAssetId::OakTree, asset_server.load(OAK_TREE_PATH));
    // gltf_asset_handles
    //     .handles
    //     .insert(GltfAssetId::PineTree, asset_server.load(PINE_TREE_PATH));
    // gltf_asset_handles
    //     .handles
    //     .insert(GltfAssetId::MapleTree, asset_server.load(MAPLE_TREE_PATH));
    // gltf_asset_handles
    //     .handles
    //     .insert(GltfAssetId::BarrenTree, asset_server.load(BARREN_TREE_PATH));
    // gltf_asset_handles
    //     .handles
    //     .insert(GltfAssetId::Lumber, asset_server.load(LUMBER_PATH));
    // gltf_asset_handles
    //     .handles
    //     .insert(GltfAssetId::WaterWell, asset_server.load(WATER_WELL_PATH));
    // gltf_asset_handles.handles.insert(
    //     GltfAssetId::PotatoPlant,
    //     asset_server.load(POTATO_PLANT_PATH),
    // );
    // gltf_asset_handles
    //     .handles
    //     .insert(GltfAssetId::Potato, asset_server.load(POTATO_PATH));
    // 
    // gltf_asset_handles
    //     .handles
    //     .insert(GltfAssetId::GrassBlade, asset_server.load(GRASS_BLADE));
    // 
    // gltf_asset_handles
    //     .handles
    //     .insert(GltfAssetId::Flower1, asset_server.load(FLOWER_1));
    // 
    // gltf_asset_handles
    //     .handles
    //     .insert(GltfAssetId::Flower2, asset_server.load(FLOWER_2));

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

#[derive(Resource)]
struct AssetsLoading(Vec<UntypedHandle>);

fn check_assets_ready(
    mut commands: Commands,
    server: Res<AssetServer>,
    loading: Res<AssetsLoading>
) {
    use bevy::asset::LoadState;

    match get_group_load_state(&server, loading.0.iter().map(|h| h.id())) {
        LoadState::Failed(_x) => {
            // one of our assets had an error
        }
        LoadState::Loaded => {
            // all assets are now ready

            // this might be a good place to transition into your in-game state

            // remove the resource to drop the tracking handles
            commands.remove_resource::<AssetsLoading>();
            // (note: if you don't have any other handles to the assets
            // elsewhere, they will get unloaded after this)
        }
        _ => {
            // NotLoaded/Loading: not fully ready yet
        }
    }
}

fn get_group_load_state(server: &AssetServer, handles: impl IntoIterator<Item = UntypedAssetId>) -> LoadState {
    let mut
    load_state
        = LoadState::Loaded;
    for handle_id in handles {
        match server.get_load_state(handle_id) {
            Some(LoadState::Loaded) => continue,
            Some(LoadState::Loading) => {

                load_state
                    = LoadState::Loading;
            }
            Some(LoadState::Failed(x)) => return LoadState::Failed(x),
            Some(LoadState::NotLoaded) => return LoadState::NotLoaded,
            None => return LoadState::NotLoaded,
        }
    }


    load_state
}