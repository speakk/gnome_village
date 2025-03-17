use bevy::prelude::*;
use bevy_easings::Ease;
use bevy_mod_async::prelude::TimingTaskExt;
use bevy_mod_async::SpawnCommandExt;
use std::time::Duration;

pub struct JuicePlugin;

#[derive(Component)]
pub struct AddTransformJuice {
    pub delay: Duration,
}

#[derive(Component, Clone, Copy)]
pub struct TransformJuice {
    pub delay: Duration,
}

impl Plugin for JuicePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, juice_new_transform);
    }
}

fn juice_new_transform(
    query: Query<(Entity, &Transform, &TransformJuice), Added<Transform>>,
    mut commands: Commands,
) {
    let cloned: Vec<_> = query.iter().map(|(e, t, tj)| (e, *t, *tj)).collect();
    for (entity, transform, transform_juice) in cloned {
        let translation = transform.translation;

        commands.spawn_task(move |cx| async move {
            cx.sleep(transform_juice.delay).await;
            cx.with_world(move |world| {
                let mut commands = world.commands();

                commands.entity(entity).insert(
                    Transform::from_xyz(translation.x, translation.y + 1.0, translation.z).ease_to(
                        Transform::from_xyz(translation.x, translation.y, translation.z),
                        bevy_easings::EaseFunction::BounceOut,
                        bevy_easings::EasingType::Once {
                            duration: Duration::from_millis(500),
                        },
                    ),
                );
                world.flush();
            })
            .await;
        });
    }
}
