use crate::features::states::AppState;
use crate::features::states::AppState::MainMenu;
use bevy::color::palettes::basic::RED;
use bevy::ecs::system::{IntoObserverSystem, ObserverSystem};
use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, button_system);
        app.add_systems(OnEnter(MainMenu), setup);
        //app.a
    }
}

/*
<palette>
  <color name="Sage" hex="a3a380" r="163" g="163" b="128" />
  <color name="Vanilla" hex="d6ce93" r="214" g="206" b="147" />
  <color name="Beige" hex="efebce" r="239" g="235" b="206" />
  <color name="Buff" hex="d8a48f" r="216" g="164" b="143" />
  <color name="Old rose" hex="bb8588" r="187" g="133" b="136" />
</palette>
 */

const NORMAL_BUTTON: Color = Color::srgb(0.31, 0.25, 0.25);
const HOVERED_BUTTON: Color = Color::srgb(0.36, 0.29, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);
const BORDER: Color = Color::srgb(0.35, 0.75, 0.35);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2d,
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
        });
}

fn create_button(
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
            BorderColor(BORDER),
            BorderRadius::all(Val::Px(5.0)),
            BackgroundColor(NORMAL_BUTTON),
        ))
        .with_child((
            Text::new("New Game"),
            TextFont {
                font: asset_server.load("fonts/ThaleahFat.ttf"),
                font_size: 23.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
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
                *color = PRESSED_BUTTON.into();
                border_color.0 = RED.into();
            }
            Interaction::Hovered => {
                //**text = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                //**text = "Button".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}
