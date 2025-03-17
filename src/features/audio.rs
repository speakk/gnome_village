use crate::features::juice::TransformJuiceFinished;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl};

pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, emit_placement_sound);
    }
}

pub fn emit_placement_sound(
    mut events: EventReader<TransformJuiceFinished>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    for event in events.read() {
        let pitch = 0.8 + (event.batch_index as f64 / event.batch_size as f64) * 0.5 + rand::random::<f64>() * 0.1;
        audio.play(asset_server.load("sounds/placement_3.ogg")).with_playback_rate(pitch);
    }
}
