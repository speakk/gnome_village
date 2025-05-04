use crate::features::states::AppState;
use crate::features::states::AppState::MainMenu;
use crate::ui::colours::{THEME_1_400, THEME_1_800};
use crate::ui::widgets::CreateButton;
use crate::ui::{widgets, FONT_BOLD};
use bevy::ecs::spawn::SpawnWith;
use bevy::picking::hover::PickingInteraction;
use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, widgets::button_colouring);
        app.add_systems(OnEnter(MainMenu), setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::from(THEME_1_400),
            ..Default::default()
        },
        IsDefaultUiCamera,
        StateScoped(MainMenu),
        Msaa::Off,
    ));

    let bold_font_handle = asset_server.load(FONT_BOLD);
    let title_image = asset_server.load("title_screen.jpg");
    
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ImageNode {
            image: title_image,
            image_mode: NodeImageMode::Stretch,
            ..Default::default()
        },
        StateScoped(MainMenu),
        Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
            parent.spawn((
                Text::new("Gnome Village".to_uppercase()),
                TextFont {
                    font: bold_font_handle,
                    font_size: 84.0,
                    ..default()
                },
                TextColor(THEME_1_800),
            ));

            parent
                .spawn(CreateButton {
                    label: "New game".to_string(),
                    ..Default::default()
                })
                .observe(
                    move |mut trigger: Trigger<Pointer<Click>>,
                          mut next_state: ResMut<NextState<AppState>>| {
                        next_state.set(AppState::LoadAssets);
                        trigger.propagate(false);
                    },
                );

            parent
                .spawn(CreateButton {
                    label: "Quit".to_string(),
                    ..Default::default()
                })
                .observe(
                    move |mut trigger: Trigger<Pointer<Click>>, mut exit: EventWriter<AppExit>| {
                        exit.write(AppExit::Success);
                        trigger.propagate(false);
                    },
                );
        })),
    ));
}
