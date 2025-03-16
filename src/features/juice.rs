use bevy::prelude::*;
use bevy_easings::Ease;
use std::time::Duration;

pub struct JuicePlugin;

#[derive(Component)]
pub struct TransformJuice;

impl Plugin for JuicePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, juice_new_transform);
    }
}

fn juice_new_transform(
    query: Query<(Entity, &Transform), (Added<Transform>, With<TransformJuice>)>,
    mut commands: Commands,
) {
    for (entity, transform) in query.iter() {
        let translation = transform.translation;
        commands.entity(entity).insert(
            Transform::from_xyz(translation.x, translation.y + 1.0, translation.z).ease_to(
                Transform::from_xyz(translation.x, translation.y, translation.z),
                bevy_easings::EaseFunction::BounceOut,
                bevy_easings::EasingType::Once {
                    duration: Duration::from_millis(500),
                },
            ),
        );
        //commands.entity(entity).insert(Transform::from_xyz(transform.translation.x, transform.translation.y, transform.translation.z).ea);
    }
}
