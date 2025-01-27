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
            .load("ui_templates/main.cob")
            .add_systems(OnEnter(LoadState::Done), build_ui);
        // app
        //     .add_systems(Update, button_system)
        //     .add_systems(PostStartup, build_ui);
    }
}

// fn build_ui(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
// ) {
//     //commands.spawn((Camera2d, IsDefaultUiCamera, Camera { order: 1, clear_color: ClearColorConfig::None, ..default()}));
// 
//     commands
//     .spawn(Node {
//         width: Val::Percent(100.0),
//         height: Val::Percent(100.0),
//         align_items: AlignItems::Center,
//         justify_content: JustifyContent::Center,
//         ..default()
//     })
//         .with_children(|parent| {
//             parent
//                 .spawn((
//                     Button,
//                     Node {
//                         width: Val::Px(150.0),
//                         height: Val::Px(65.0),
//                         border: UiRect::all(Val::Px(5.0)),
//                         // horizontally center child text
//                         justify_content: JustifyContent::Center,
//                         // vertically center child text
//                         align_items: AlignItems::Center,
//                         ..default()
//                     },
//                     BorderColor(Color::BLACK),
//                     BorderRadius::MAX,
//                     BackgroundColor(NORMAL_BUTTON),
//                 ))
//                 .with_child((
//                     Text::new("Button"),
//                     TextFont {
//                         font: asset_server.load("fonts/FiraSans-Bold.ttf"),
//                         font_size: 33.0,
//                         ..default()
//                     },
//                     TextColor(Color::srgb(0.9, 0.9, 0.9)),
//                 ));
//         });
//     
// }
// 
// fn button_system(
//     mut interaction_query: Query<
//         (
//             &Interaction,
//             &mut BackgroundColor,
//             &mut BorderColor,
//             &Children,
//         ),
//         (Changed<Interaction>, With<Button>),
//     >,
//     mut text_query: Query<&mut Text>,
// ) {
//     for (interaction, mut color, mut border_color, children) in &mut interaction_query {
//         let mut text = text_query.get_mut(children[0]).unwrap();
//         match *interaction {
//             Interaction::Pressed => {
//                 **text = "Press".to_string();
//                 *color = PRESSED_BUTTON.into();
//                 border_color.0 = RED.into();
//             }
//             Interaction::Hovered => {
//                 **text = "Hover".to_string();
//                 *color = HOVERED_BUTTON.into();
//                 border_color.0 = Color::WHITE;
//             }
//             Interaction::None => {
//                 **text = "Button".to_string();
//                 *color = NORMAL_BUTTON.into();
//                 border_color.0 = Color::BLACK;
//             }
//         }
//     }
// }

fn build_ui(
    mut commands: Commands,
    mut scene_loader: ResMut<SceneLoader>,
) {
    //commands.spawn((Camera2d, IsDefaultUiCamera, Camera { order: 1, clear_color: ClearColorConfig::None, ..default()}));
    commands.ui_root().load_scene(("ui_templates/main.cob", "main_scene"), &mut scene_loader);
}