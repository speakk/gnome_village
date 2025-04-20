use crate::features::states::AppState;
use crate::features::states::AppState::MainMenu;
use crate::ui::colours::{THEME_1_400, THEME_1_800};
use crate::ui::widgets::{CreateButtonParams, WidgetSystems};
use bevy::prelude::*;
use crate::ui::{widgets, FONT_BOLD};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, widgets::button_colouring);
        app.add_systems(OnEnter(MainMenu), setup);
        //app.init_resource::<ButtonImage>();
        //app.a
    }
}

// #[derive(Resource, Default)]
// struct ButtonImage(Option<Handle<Image>>);
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    widget_systems: Res<WidgetSystems>, //mut button_image: ResMut<ButtonImage>,
) {
    let button_image: Handle<Image> = asset_server.load("textures/button_1.png");
    //button_image.0 = Some(image);

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

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            StateScoped(MainMenu),
        ))
        .with_children(move |parent| {
            parent.spawn((
                Text::new("Gnome Village".to_uppercase()),
                TextFont {
                    font: asset_server.load(FONT_BOLD),
                    font_size: 84.0,
                    ..default()
                },
                TextColor(THEME_1_800),
            ));

            let button_system_id = widget_systems.button.clone();

            let button_entity = parent
                .spawn_empty()
                .observe(
                    move |mut trigger: Trigger<Pointer<Click>>,
                          mut next_state: ResMut<NextState<AppState>>| {
                        next_state.set(AppState::Preload);
                        trigger.propagate(false);
                    },
                )
                .id();

            parent.enqueue_command(move |world: &mut World| {
                let mut commands = world.commands();
                commands.run_system_with_input(
                    button_system_id.clone(),
                    CreateButtonParams {
                        label: "New game".to_string(),
                        button_entity,
                        ..Default::default()
                    },
                );
            });

            let button_entity = parent
                .spawn_empty()
                .observe(
                    move |mut trigger: Trigger<Pointer<Click>>, mut exit: EventWriter<AppExit>| {
                        exit.send(AppExit::Success);
                        trigger.propagate(false);
                    },
                )
                .id();

            parent.enqueue_command(move |world: &mut World| {
                let mut commands = world.commands();
                commands.run_system_with_input(
                    button_system_id,
                    CreateButtonParams {
                        label: "Quit".to_string(),
                        button_entity,
                        ..Default::default()
                    },
                );
            });
        });
}

