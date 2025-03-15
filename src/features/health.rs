use bevy::prelude::*;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, health_system);
        app.add_observer(death_observer);
    }
}

#[derive(Event)]
pub struct HealthDepleted;

fn health_system(query: Query<(Entity, &Health), Changed<Health>>, mut commands: Commands) {
    for (entity, health) in query.iter() {
        if health.health <= 0.0 {
            commands.entity(entity).trigger(HealthDepleted);
        }
    }
}

fn death_observer(trigger: Trigger<HealthDepleted>, mut commands: Commands) {
    commands.entity(trigger.entity()).despawn();
}

#[derive(Component)]
pub struct Health {
    pub max_health: f32,
    pub health: f32,
}

impl Default for Health {
    fn default() -> Self {
        Self::new(1.0)
    }
}

impl Health {
    pub fn new(max_health: f32) -> Self {
        Self {
            max_health,
            health: max_health,
        }
    }

    pub fn reduce_health(&mut self, amount: f32) {
        self.health -= amount;
    }
}
