use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl};
use crate::features::states::AppState;

pub fn music_plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::MainMenu), play_main_menu_music);
    app.add_systems(OnEnter(AppState::InGame), play_in_game_music);
}

pub fn play_main_menu_music(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio.stop();
    audio.play(asset_server.load("music/speak-on_a_branch.mp3")).looped();
}

pub fn play_in_game_music(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio.stop();
    audio.play(asset_server.load("music/ambient_5.mp3")).looped();
}