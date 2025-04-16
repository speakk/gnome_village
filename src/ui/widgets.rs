use crate::ui::colours::THEME_1_800;
use bevy::ecs::system::SystemId;
use bevy::prelude::*;
use bevy::ui::widget::NodeImageMode;
use crate::features::states::AppState;

pub fn widget_plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::MainMenu), setup_button_image);
    app.init_resource::<WidgetSystems>();
    app.init_resource::<ButtonImage>();
}

#[derive(Resource)]
pub struct WidgetSystems {
    pub button: SystemId<In<CreateButtonParams>>,
}

#[derive(Resource, Default)]
pub struct ButtonImage(Option<Handle<Image>>);

fn setup_button_image(
    asset_server: Res<AssetServer>,
    mut button_image: ResMut<ButtonImage>,
) {
    button_image.0 = Some(asset_server.load("textures/button_1.png"));
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
}

pub fn create_button_system(
    In(CreateButtonParams {
        label,
        button_entity,
    }): In<CreateButtonParams>,
    button_image: Res<ButtonImage>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let slicer = TextureSlicer {
        border: BorderRect::square(32.0),
        center_scale_mode: SliceScaleMode::Stretch,
        sides_scale_mode: SliceScaleMode::Stretch,
        max_corner_scale: 1.0,
    };

    commands
        .entity(button_entity)
        .insert((
            Button,
            ImageNode {
                image: button_image.0.clone().unwrap(),
                image_mode: NodeImageMode::Sliced(slicer.clone()),
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
                font: asset_server.load("fonts/ThaleahFat.ttf"),
                font_size: 42.0,
                ..default()
            },
            TextColor(THEME_1_800),
        ));
}
