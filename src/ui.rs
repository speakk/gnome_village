use bevy::color::palettes::basic::RED;
use bevy::prelude::*;
use bevy_cobweb_ui::prelude::*;
use crate::ui::main_action_buttons::spawn_main_action_buttons;

pub mod main_action_buttons;

pub struct UiPlugin;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(CobwebUiPlugin)
            .load("ui_templates/manifest.cob")
            .add_systems(OnEnter(LoadState::Done), build_ui);
        // app
        //     .add_systems(Update, button_system)
        //     .add_systems(PostStartup, build_ui);
    }
}

fn build_ui(
    mut commands: Commands,
    mut scene_loader: ResMut<SceneLoader>,
) {
    //commands.spawn((Camera2d, IsDefaultUiCamera, Camera { order: 1, clear_color: ClearColorConfig::None, ..default()}));

    let buttons = vec![
        "Build", "Order"
    ];

    commands.ui_root().load_scene_and_edit(("main", "main_scene"), &mut scene_loader, |scene| {
        for button in buttons {
            scene.get( "buttons_container").load_scene_and_edit(("button", "button"), |button_scene| {
                button_scene.get("text").update_text(button);
            });
        }
    });
}