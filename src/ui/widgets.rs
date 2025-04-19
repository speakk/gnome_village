use crate::ui::colours::THEME_1_800;
use bevy::ecs::system::SystemId;
use bevy::prelude::*;
use bevy::ui::widget::NodeImageMode;
use crate::features::states::AppState;

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
    pub font_size: f32
}

impl Default for CreateButtonParams {
    fn default() -> Self {
        CreateButtonParams {
            label: "".to_string(),
            button_entity: Entity::PLACEHOLDER,
            font: "fonts/ThaleahFat.ttf".to_string(),
            font_size: 42.0
        }
    }
}

pub fn create_button_system(
    In(CreateButtonParams {
        label,
        button_entity,
        font,
        font_size
    }): In<CreateButtonParams>,
    button_data: Res<ButtonData>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    commands
        .entity(button_entity)
        .insert_if_new((
            Button,
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
