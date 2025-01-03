use bevy::prelude::*;

use crate::color;
use crate::consts::*;
use crate::consts_private::*;
use crate::physics::{self, StartupDamping};
use crate::random;
use crate::AverageEK;

#[derive(Component)]
pub struct ParticleDensity(pub f32);

#[derive(Component)]
pub struct ParticlePressure(pub f32);

#[derive(Component)]
pub struct ParticlePosition(pub Vec2);

#[derive(Component)]
pub struct PrevParticlePosition(pub Option<Vec2>);

#[derive(Component)]
pub struct PredictedParticlePosition(pub Vec2);

#[derive(Component)]
pub struct ParticleVelocity(pub Vec2);

#[derive(Component)]
pub struct ParticleAcceleration(pub Vec2);

pub fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let circle = meshes.add(Circle::new(PARTICLE_SCREEN_RADIUS));

    for _ in 0..NUM_PARTICLES {
        let (x, y) = random::point_in_box((
            PHYSICAL_HALF_SIZE.0 * 0.5,
            PHYSICAL_HALF_SIZE.1 * 0.5,
        ));

        commands.spawn((
            // Animation properties.
            Mesh2d(circle.clone()),
            MeshMaterial2d(materials.add(color::for_velocity(0.0))),
            Transform::from_xyz(x * SCREEN_FACTOR, y * SCREEN_FACTOR, 1.0),
            // Physical properties.
            ParticleDensity(0.0),
            ParticlePressure(0.0),
            PrevParticlePosition(None),
            ParticlePosition(Vec2 {x, y}),
            PredictedParticlePosition(Vec2::ZERO),
            ParticleVelocity(Vec2::ZERO),
            ParticleAcceleration(Vec2::ZERO),
        ));
    }
}

pub fn predict_positions(
    mut particles: Query<(
        &PrevParticlePosition,
        &ParticlePosition,
        &mut PredictedParticlePosition,
        &ParticleVelocity,
        &ParticleAcceleration,
    )>,
) {
    // Use a constant timestep for position prediction.
    const DT: f32 = 1.0 / 60.0;
    for (
        PrevParticlePosition(prev_x),
        ParticlePosition(x),
        mut next_x,
        ParticleVelocity(v),
        ParticleAcceleration(a),
    ) in &mut particles {
        let res = physics::verlet(
            prev_x,
            x,
            v,
            a,
            DT,
        );
        next_x.0 = res.x;
    }
}

pub fn update_densities_and_pressures(
    mut particles: Query<(
        Entity,
        &PredictedParticlePosition,
        &mut ParticleDensity,
        &mut ParticlePressure,
    )>,
    positions: Query<(Entity, &ParticlePosition)>,
    damping: Res<StartupDamping>,
) {
    // For each particle.
    for (
        entity,
        PredictedParticlePosition(pred_pos),
        mut density,
        mut pressure,
    ) in &mut particles {
        // Start with the density from the edge of the container.
        let mut sum = if EDGE_REPULSION {
            physics::compute_edge_density(pred_pos)
        } else {
            0.0
        };
        // Sum the density contributions of all particles on that position.
        for (other_entity, ParticlePosition(pos)) in positions.iter() {
            // Ignore the density contribution of this particle.
            if other_entity == entity {
                continue;
            }
            let displacement_squared = (pred_pos - pos).length_squared();
            sum += DENSITY_KERNEL.influence(displacement_squared);
        }
        // Finally, add the density contribution of the particle itself.
        density.0 = sum + DENSITY_FACTOR;
        pressure.0 = physics::density_to_pressure(
            density.0, damping.0 * PRESSURE_MULTIPLIER,
        );
    }
}

pub fn update_accelerations(
    mut accelerations: Query<(Entity, &mut ParticleAcceleration)>,
    particles: Query<(
        Entity,
        &PredictedParticlePosition,
        &ParticleVelocity,
        &ParticlePressure,
        &ParticleDensity,
    )>,
    damping: Res<StartupDamping>,
) {
    // For each particle.
    for (entity, mut acceleration) in &mut accelerations {
        // Get the predicted position of and density at that particle.
        let (
            _,
            PredictedParticlePosition(pos_x),
            ParticleVelocity(vel_x),
            ParticlePressure(pressure_x),
            ParticleDensity(density_x),
        ) = particles.get(entity).unwrap();

        let mut pressure_gradient = Vec2::ZERO;
        let mut viscosity_force = Vec2::ZERO;
        // Sum the acceleration contributions of all particles on that position.
        for (
            other_entity,
            PredictedParticlePosition(pos_i),
            ParticleVelocity(vel_i),
            ParticlePressure(pressure_i),
            ParticleDensity(density_i),
        ) in particles.iter() {
            if other_entity == entity {
                continue;
            }
            let displacement = pos_x - pos_i;

            // Compute pressure gradient contribution.
            let shared_pressure = 0.5 * (pressure_x + pressure_i);
            pressure_gradient += shared_pressure * DENSITY_KERNEL.gradient(displacement) / density_i;

            // Compute viscosity contribution.
            viscosity_force +=
                (vel_i - vel_x) * VISCOSITY_KERNEL.influence(displacement.length_squared());
        }

        // Compute acceleration.
        let mut acc = damping.0 * pressure_gradient / density_x + viscosity_force * VISCOSITY;
        if EDGE_REPULSION {
            acc += physics::compute_edge_acceleration(
                &pos_x,
                *density_x,
                damping.0 * PRESSURE_MULTIPLIER,
            );
        }
        acc.y -= GRAVITY_FORCE;

        acceleration.0 = acc;
    }
}

// Compute the allowed range for the centre of a particle.
pub const PHYSICAL_SIZE: (f32, f32) = (
    BOX_SIZE_F.0 / SCREEN_FACTOR, BOX_SIZE_F.1 / SCREEN_FACTOR,
);
pub const PHYSICAL_HALF_SIZE: (f32, f32) = (
    PHYSICAL_SIZE.0 * 0.5, PHYSICAL_SIZE.1 * 0.5,
);

pub fn update_positions(
    time: Res<Time>,
    mut particles: Query<(
        &mut Transform,
        &mut PrevParticlePosition,
        &mut ParticlePosition,
        &mut ParticleVelocity,
        &ParticleAcceleration,
    )>,
    mut average_ek: ResMut<AverageEK>,
) {
    let dt = time.delta_secs();
    let mut ek_sum = 0.0;
    for (
        mut transform,
        mut prev_x,
        mut x,
        mut v,
        ParticleAcceleration(a),
    ) in &mut particles {
        // Compute the next position.

        let res = physics::verlet(
            &prev_x.0, &x.0, &v.0, a, dt,
        );

        // Add to EK sum.
        ek_sum += res.v.length_squared();

        // Set variables to new values.
        v.0 = res.v;

        if res.moved || prev_x.0.is_some() {
            prev_x.0 = Some(res.prev_x);
        }
        x.0 = res.x;

        // Propagate position changes to transform to update animation.
        transform.translation.x = res.x.x * SCREEN_FACTOR;
        transform.translation.y = res.x.y * SCREEN_FACTOR;
    }
    // Set EK value.
    average_ek.0 = 0.5 * ek_sum / NUM_PARTICLES as f32;
}

pub fn update_colors(
    particles: Query<(&MeshMaterial2d<ColorMaterial>, &ParticleVelocity)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (color, velocity) in &particles {
        materials.get_mut(color).unwrap().color = color::for_velocity(velocity.0.length());
    }
}
