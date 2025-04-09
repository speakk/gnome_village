use crate::features::states::AppState;
use crate::features::states::AppState::MainMenu;
use crate::ui::colours::{THEME_1_400, THEME_1_800, THEME_2_400, THEME_2_600, THEME_2_DEFAULT};
use bevy::ecs::system::{IntoObserverSystem, ObserverSystem};
use bevy::prelude::*;
use bevy::ui::widget::NodeImageMode;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, button_system);
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
    //mut button_image: ResMut<ButtonImage>,
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
        .with_children(|parent| {
            parent.spawn((
                Text::new("Gnome Village".to_uppercase()),
                TextFont {
                    font: asset_server.load("fonts/ThaleahFat.ttf"),
                    font_size: 84.0,
                    ..default()
                },
                TextColor(THEME_1_800),
            ));

            create_button(
                "New game".to_string(),
                parent,
                button_image.clone(),
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
                button_image.clone(),
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
    image: Handle<Image>,
    asset_server: &Res<AssetServer>,
    observe_logic: impl ObserverSystem<Pointer<Click>, (), ()> + 'static,
) {
    let slicer = TextureSlicer {
        border: BorderRect::square(32.0),
        center_scale_mode: SliceScaleMode::Stretch,
        sides_scale_mode: SliceScaleMode::Stretch,
        max_corner_scale: 1.0,
    };

    parent
        .spawn((
            Button,
            ImageNode {
                image: image.clone(),
                image_mode: NodeImageMode::Sliced(slicer.clone()),
                ..default()
            },
            Node {
                width: Val::Px(230.0),
                height: Val::Px(80.0),
                //border: UiRect::all(Val::Px(2.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            //BorderColor(THEME_1_400),
            //BorderRadius::all(Val::Px(5.0)),
            //BackgroundColor(THEME_2_DEFAULT),
        ))
        .with_child((
            Text::new(label),
            TextFont {
                font: asset_server.load("fonts/ThaleahFat.ttf"),
                font_size: 42.0,
                ..default()
            },
            TextColor(THEME_1_800),
        ))
        .observe(observe_logic);
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut ImageNode, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut image_node, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                image_node.color = THEME_2_400;
            }
            Interaction::Hovered => {
                image_node.color = THEME_2_600;
            }
            Interaction::None => {
                image_node.color = THEME_2_DEFAULT;
            }
        }
    }
}
