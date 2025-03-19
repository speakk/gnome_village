use crate::features::particles::{ParticleHandles, ParticleType};
use bevy::asset::AssetContainer;
use bevy::prelude::*;
use bevy::render::mesh::{SphereKind, SphereMeshBuilder};
use bevy_hanabi::prelude::*;

pub(super) fn setup_light_sparkle(
    mut effects: ResMut<Assets<EffectAsset>>,
    mut particle_handles: ResMut<ParticleHandles>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // Define a color gradient from red to transparent black
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(1., 0.7, 0., 1.));
    gradient.add_key(0.3, Vec4::new(1., 0.4, 0., 0.5));
    gradient.add_key(1.0, Vec4::splat(0.));

    let mesh = meshes.add(SphereMeshBuilder::new(0.05, SphereKind::Ico { subdivisions: 1 }).build());

    // Create a new expression module
    let mut module = Module::default();

    // On spawn, randomly initialize the position of the particle
    // to be over the surface of a sphere of radius 2 units.
    let init_pos = SetPositionSphereModifier {
        center: module.lit(Vec3::new(0., 0.0, 0.)),
        radius: module.lit(0.1),
        dimension: ShapeDimension::Surface,
    };

    // Also initialize a radial initial velocity to 6 units/sec
    // away from the (same) sphere center.
    let init_vel = SetVelocitySphereModifier {
        center: module.lit(Vec3::new(0., -0.5, 0.)),
        speed: module.lit(0.4),
    };

    let writer = ExprWriter::new();

    let init_size = SetAttributeModifier::new(
        Attribute::F32_0,
        (writer.rand(ScalarType::Float) * writer.lit(0.5) + writer.lit(0.5)).expr(),
    );

    // Make the particles shrink over time.
    let update_size = SetAttributeModifier::new(
        Attribute::SIZE,
        writer
            .attr(Attribute::F32_0)
            .mul(
                writer
                    .lit(10.0)
                    .sub((writer.attr(Attribute::AGE)).mul(writer.lit(0.98)))
                    .max(writer.lit(0.0)),
            )
            .expr(),
    );

    // Initialize the total lifetime of the particle, that is
    // the time for which it's simulated and rendered. This modifier
    // is almost always required, otherwise the particles won't show.
    let lifetime = module.lit(0.6); // literal value "10.0"
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    // Every frame, add a gravity-like acceleration downward
    let accel = module.lit(Vec3::new(0., 3., 0.));
    let update_accel = AccelModifier::new(accel);

    // Create the effect asset
    let effect = EffectAsset::new(
        // Maximum number of particles alive at a time
        400,
        // Spawn at a rate of 5 particles per second
        //SpawnerSettings::rate(5.0.into()),
        Spawner::rate(40.0.into()),
        // Move the expression module into the asset
        module,
    )
    .with_name("LightSparkles")
    .init(init_pos)
    .init(init_vel)
    .init(init_size)
    .init(init_lifetime)
    .update(update_accel)
    .mesh(mesh.clone())
    //.update(update_size)
    // Render the particles with a color gradient over their
    // lifetime. This maps the gradient key 0 to the particle spawn
    // time, and the gradient key 1 to the particle death (10s).
    .render(ColorOverLifetimeModifier { gradient });

    // Insert into the asset system
    let effect_handle = effects.add(effect);
    particle_handles
        .0
        .insert(ParticleType::LightSparkle, effect_handle);
}
