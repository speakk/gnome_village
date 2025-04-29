use crate::features::states::AppState;
use crate::ui::colours::{THEME_1_800, THEME_2_400, THEME_2_600, THEME_2_DEFAULT};
use crate::ui::FONT_BOLD;
use bevy::ecs::system::SystemId;
use bevy::prelude::*;
use bevy::ui::widget::NodeImageMode;

pub fn widget_plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::MainMenu), setup_button_data);
    app.add_observer(create_button);
    app.init_resource::<ButtonData>();
}

#[derive(Resource, Default)]
pub struct ButtonData {
    image: Option<Handle<Image>>,
    slicer: Option<TextureSlicer>,
}

fn setup_button_data(asset_server: Res<AssetServer>, mut button_data: ResMut<ButtonData>) {
    *button_data = ButtonData {
        image: Some(asset_server.load("textures/button_1.png")),
        slicer: Some(TextureSlicer {
            border: BorderRect::all(32.0),
            center_scale_mode: SliceScaleMode::Stretch,
            sides_scale_mode: SliceScaleMode::Stretch,
            max_corner_scale: 1.0,
        }),
    }
}

#[derive(Clone, Copy)]
pub struct ColorDefinition {
    pub normal: Color,
    pub hovered: Color,
    pub pressed: Color,
}

#[derive(Component)]
pub struct ButtonColor(pub ColorDefinition);

#[derive(Component, Clone)]
pub struct CreateButton {
    pub label: String,
    pub font: String,
    pub font_size: f32,
    pub color_definition: ColorDefinition,
}

impl Default for CreateButton {
    fn default() -> Self {
        CreateButton {
            label: "".to_string(),
            font: FONT_BOLD.parse().unwrap(),
            font_size: 42.0,
            color_definition: ColorDefinition {
                normal: THEME_2_DEFAULT,
                hovered: THEME_2_600,
                pressed: THEME_2_400,
            },
        }
    }
}

pub fn create_button(
    trigger: Trigger<OnAdd, CreateButton>,
    query: Query<&CreateButton>,
    button_data: Res<ButtonData>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let button_entity = trigger.target();
    let CreateButton {
        color_definition,
        label,
        font,
        font_size,
    } = query.get(button_entity).unwrap().clone();

    commands
        .entity(button_entity)
        .insert_if_new((
            Button,
            ButtonColor(color_definition),
            ImageNode {
                image: button_data.image.clone().unwrap(),
                color: color_definition.normal,
                image_mode: NodeImageMode::Sliced(button_data.slicer.clone().unwrap()),
                ..default()
            },
            Node {
                width: Val::Px(230.0),
                height: Val::Px(80.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::all(Val::Px(10.0)),
                ..default()
            },
        ))
        .with_child((
            Text::new(label),
            TextFont {
                font: asset_server.load(font),
                font_size,
                ..default()
            },
            TextColor(THEME_1_800),
        ));
}

pub fn button_colouring(
    mut interaction_query: Query<
        (&Interaction, &mut ImageNode, &Children, &ButtonColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut image_node, children, button_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                image_node.color = button_color.0.pressed;
            }
            Interaction::Hovered => {
                image_node.color = button_color.0.hovered;
            }
            Interaction::None => {
                image_node.color = button_color.0.normal;
            }
        }
    }
}
