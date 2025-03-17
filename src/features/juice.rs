use bevy::prelude::*;
use bevy_easings::Ease;
use bevy_mod_async::prelude::TimingTaskExt;
use bevy_mod_async::SpawnCommandExt;
use std::time::Duration;

pub struct JuicePlugin;

#[derive(Component, Clone, Copy)]
pub struct AddTransformJuice {
    pub batch_index: usize,
    pub batch_size: usize,
}

#[derive(Component, Clone, Copy)]
pub struct TransformJuice {
    pub batch_index: usize,
    pub batch_size: usize,
}

impl From<AddTransformJuice> for TransformJuice {
    fn from(add_transform_juice: AddTransformJuice) -> Self {
        TransformJuice {
            batch_size: add_transform_juice.batch_size,
            batch_index: add_transform_juice.batch_index,
        }
    }
}

impl Plugin for JuicePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, juice_new_transform)
            .add_event::<TransformJuiceFinished>();
    }
}

#[derive(Event)]
pub struct TransformJuiceFinished {
    pub batch_index: usize,
    pub batch_size: usize,
}

impl From<TransformJuice> for TransformJuiceFinished {
    fn from(transform_juice: TransformJuice) -> Self {
        TransformJuiceFinished {
            batch_size: transform_juice.batch_size,
            batch_index: transform_juice.batch_index,
        }
    }
}

fn juice_new_transform(
    query: Query<(Entity, &Transform, &TransformJuice), Added<Transform>>,
    mut commands: Commands,
) {
    let cloned: Vec<_> = query.iter().map(|(e, t, tj)| (e, *t, *tj)).collect();
    for (entity, transform, transform_juice) in cloned {
        let translation = transform.translation;
        let duration = Duration::from_millis(transform_juice.batch_index as u64 * 20);
        
        
        commands.spawn_task(move |cx| async move {
            cx.sleep(duration).await;
            
            const TRANSFORM_DURATION: Duration = Duration::from_millis(500);
            
            cx.with_world(move |world| {
                let mut commands = world.commands();

                commands.entity(entity).insert(
                    Transform::from_xyz(translation.x, translation.y + 1.0, translation.z).ease_to(
                        Transform::from_xyz(translation.x, translation.y, translation.z),
                        bevy_easings::EaseFunction::BounceOut,
                        bevy_easings::EasingType::Once {
                            duration: TRANSFORM_DURATION,
                        },
                    ),
                );
                world.flush();
            })
            .await;

            //cx.sleep(duration).await;
            cx.with_world(move |world| {
                world.send_event(TransformJuiceFinished::from(transform_juice));
                world.flush();
            }).await;
        });
    }
}
