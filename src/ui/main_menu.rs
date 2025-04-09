use crate::features::states::AppState;
use crate::features::states::AppState::MainMenu;
use bevy::color::palettes::basic::RED;
use bevy::ecs::system::{IntoObserverSystem, ObserverSystem};
use bevy::prelude::*;
use crate::ui::colours::{THEME_1_100, THEME_1_200, THEME_1_400, THEME_2_100, THEME_2_200, THEME_2_600, THEME_2_DEFAULT, THEME_3_100, THEME_3_300, THEME_3_DEFAULT, THEME_4_DEFAULT};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, button_system);
        app.add_systems(OnEnter(MainMenu), setup);
        //app.a
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
        .with_children(|parent| {
            create_button(
                "New game".to_string(),
                parent,
                &asset_server,
                IntoObserverSystem::into_system(
                    move |mut trigger: Trigger<Pointer<Click>>,
                          mut next_state: ResMut<NextState<AppState>>| {
                        next_state.set(AppState::Preload);
                        trigger.propagate(false);
                    },
                ),
            );

            create_button(
                "Quit".to_string(),
                parent,
                &asset_server,
                IntoObserverSystem::into_system(
                    move |mut trigger: Trigger<Pointer<Click>>, mut exit: EventWriter<AppExit>| {
                        exit.send(AppExit::Success);
                        trigger.propagate(false);
                    },
                ),
            );
        });
}

fn create_button(
    label: String,
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    observe_logic: impl ObserverSystem<Pointer<Click>, (), ()> + 'static,
) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(150.0),
                height: Val::Px(65.0),
                border: UiRect::all(Val::Px(2.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BorderColor(THEME_1_400),
            BorderRadius::all(Val::Px(5.0)),
            BackgroundColor(THEME_2_DEFAULT),
        ))
        .with_child((
            Text::new(label),
            TextFont {
                font: asset_server.load("fonts/ThaleahFat.ttf"),
                font_size: 23.0,
                ..default()
            },
            TextColor(THEME_1_200),
        ))
        .observe(observe_logic);
}

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                //**text = "Press".to_string();
                *color = THEME_2_200.into();
                border_color.0 = RED.into();
            }
            Interaction::Hovered => {
                //**text = "Hover".to_string();
                *color = THEME_2_600.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                //**text = "Button".to_string();
                *color = THEME_2_DEFAULT.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}
