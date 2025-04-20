use crate::ui::colours::{THEME_1_800, THEME_2_400, THEME_2_600, THEME_2_DEFAULT};
use bevy::ecs::system::SystemId;
use bevy::prelude::*;
use bevy::ui::widget::NodeImageMode;
use crate::features::states::AppState;
use crate::ui::FONT_BOLD;

pub fn widget_plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::MainMenu), setup_button_data);
    app.init_resource::<WidgetSystems>();
    app.init_resource::<ButtonData>();
}

#[derive(Resource)]
pub struct WidgetSystems {
    pub button: SystemId<In<CreateButtonParams>>,
}

#[derive(Resource, Default)]
pub struct ButtonData {
    image: Option<Handle<Image>>,
    slicer: Option<TextureSlicer>
}

fn setup_button_data(
    asset_server: Res<AssetServer>,
    mut button_data: ResMut<ButtonData>,
) {
    *button_data = ButtonData {
        image: Some(asset_server.load("textures/button_1.png")),
        slicer: Some(TextureSlicer {
            border: BorderRect::square(32.0),
            center_scale_mode: SliceScaleMode::Stretch,
            sides_scale_mode: SliceScaleMode::Stretch,
            max_corner_scale: 1.0,
        })
    }
}

pub struct ColorDefinition {
    pub normal: Color,
    pub hovered: Color,
    pub pressed: Color,
}

#[derive(Component)]
pub struct ButtonColor(pub ColorDefinition);

impl FromWorld for WidgetSystems {
    fn from_world(world: &mut World) -> Self {
        WidgetSystems {
            button: world.register_system(create_button_system),
        }
    }
}

pub struct CreateButtonParams {
    pub label: String,
    pub button_entity: Entity,
    pub font: String,
    pub font_size: f32,
    pub color_definition: ColorDefinition
}

impl Default for CreateButtonParams {
    fn default() -> Self {
        CreateButtonParams {
            label: "".to_string(),
            button_entity: Entity::PLACEHOLDER,
            font: FONT_BOLD.parse().unwrap(),
            font_size: 42.0,
            color_definition: ColorDefinition {
                normal: THEME_2_DEFAULT,
                hovered: THEME_2_600,
                pressed: THEME_2_400
            }
        }
    }
}

pub fn create_button_system(
    In(CreateButtonParams {
        label,
        button_entity,
        font,
        font_size,
        color_definition
    }): In<CreateButtonParams>,
    button_data: Res<ButtonData>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    commands
        .entity(button_entity)
        .insert_if_new((
            Button,
            ButtonColor(color_definition),
            ImageNode {
                image: button_data.image.clone().unwrap(),
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
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut image_node, children, button_color) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
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