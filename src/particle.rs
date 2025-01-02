use bevy::prelude::*;

use crate::color;
use crate::consts::*;
use crate::consts_private::*;
use crate::kernel::{self, Kernel};
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
            PHYSICAL_HALF_SIZE.0,
            PHYSICAL_HALF_SIZE.1,
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
            ParticleVelocity(Vec2::ZERO),
            ParticleAcceleration(Vec2::ZERO),
        ));
    }
}

pub fn update_densities_and_pressures(
    mut densities: Query<(Entity, &mut ParticleDensity, &mut ParticlePressure)>,
    positions: Query<&ParticlePosition>,
    damping: Res<StartupDamping>,
) {
    // For each particle.
    for (
        entity,
        mut density,
        mut pressure,
    ) in &mut densities {
        // Get the position of that particle.
        let ParticlePosition(sample_point) = positions.get(entity).unwrap();
        // Start with the density from the edge of the container.
        let mut sum = if EDGE_REPULSION {
            physics::compute_edge_density(sample_point)
        } else {
            0.0
        };
        // Sum the density contributions of all particles on that position.
        for ParticlePosition(pos) in positions.iter() {
            if std::ptr::addr_eq(sample_point, pos) {
                continue;
            }
            let displacement_squared = (sample_point - pos).length_squared();
            sum += match DENSITY_KERNEL {
                Kernel::Smooth6 => kernel::smooth6(displacement_squared),
                Kernel::Spiky2 => kernel::spiky2(displacement_squared),
            }
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
    particles: Query<(&ParticlePosition, &ParticlePressure, &ParticleDensity)>,
    damping: Res<StartupDamping>,
) {
    // For each particle.
    for (entity, mut acceleration) in &mut accelerations {
        // Get the position of and density at that particle.
        let (
            ParticlePosition(pos_x),
            ParticlePressure(pressure_x),
            ParticleDensity(density_x),
        ) = particles.get(entity).unwrap();
        // Start with any acceleration from the edge of the box.
        let mut acc = if EDGE_REPULSION {
            physics::compute_edge_acceleration(
                &pos_x,
                *density_x,
                damping.0 * PRESSURE_MULTIPLIER,
            )
        } else {
            Vec2::ZERO
        };

        // Sum the acceleration contributions of all particles on that position.
        for (
            ParticlePosition(pos_i),
            ParticlePressure(pressure_i),
            ParticleDensity(density_i),
        ) in particles.iter() {
            if std::ptr::addr_eq(pos_x, pos_i) {
                continue;
            }

            let shared_pressure = 0.5 * (pressure_x + pressure_i);
            acc += shared_pressure * kernel::grad_spiky2(pos_x - pos_i) / density_i;
        }
        // Update acceleration. Damp it initially to prevent large values.
        acceleration.0 = acc * damping.0;
        acceleration.0.y -= GRAVITY_FORCE;
    }
}

// Compute the allowed range for the centre of a particle.
pub const PARTICLE_RADIUS: f32 = PARTICLE_SCREEN_RADIUS / SCREEN_FACTOR;
pub const PHYSICAL_SIZE: (f32, f32) = (
    BOX_SIZE_F.0 / SCREEN_FACTOR, BOX_SIZE_F.1 / SCREEN_FACTOR,
);
pub const PHYSICAL_HALF_SIZE: (f32, f32) = (
    PHYSICAL_SIZE.0 * 0.5, PHYSICAL_SIZE.1 * 0.5,
);
const PARTICLE_CENTRE_BOUND: (f32, f32) = (
    PHYSICAL_HALF_SIZE.0 - PARTICLE_RADIUS, PHYSICAL_HALF_SIZE.1 - PARTICLE_RADIUS,
);

pub fn verlet_integrate(
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
        mut prev_x_var,
        mut x_var,
        mut v_var,
        ParticleAcceleration(a),
    ) in &mut particles {
        // Compute the next position.
        let x = &mut x_var.0;
        let v = &v_var.0;

        let delta_x = match prev_x_var.0 {
            None => {
                // N = 1 integration case.
                *v * dt + 0.5 * a * dt * dt
            },
            Some(prev_x) => {
                *x - prev_x + a * dt * dt
            },
        };
        let mut next_x = *x + delta_x;
        // Perform boundary checks.

        let no_move = delta_x.length() < std::f32::EPSILON;
        let mut new_v = if no_move {
            *v
        } else {
            (next_x - *x) / dt
        };
        boundary_check(
            &mut x.x,
            &mut next_x.x,
            &mut new_v.x,
            -PARTICLE_CENTRE_BOUND.0, PARTICLE_CENTRE_BOUND.0,
        );
        boundary_check(
            &mut x.y,
            &mut next_x.y,
            &mut new_v.y,
            -PARTICLE_CENTRE_BOUND.1, PARTICLE_CENTRE_BOUND.1,
        );

        // Add to EK sum.
        ek_sum += new_v.length_squared();

        // Set variables to new values.
        v_var.0 = new_v;

        if !no_move || prev_x_var.0.is_some() {
            prev_x_var.0 = Some(*x);
        }
        x_var.0 = next_x;

        // Propagate position changes to transform to update animation.
        transform.translation.x = next_x.x * SCREEN_FACTOR;
        transform.translation.y = next_x.y * SCREEN_FACTOR;
    }
    // Set EK value.
    average_ek.0 = 0.5 * ek_sum / NUM_PARTICLES as f32;
}

fn boundary_check(
    prev_x: &mut f32,
    new_x: &mut f32,
    v: &mut f32,
    low: f32,
    high: f32,
) {
    if *new_x < low {
        *prev_x = 2.0 * low - *prev_x;
        *new_x = 2.0 * low - *new_x;
        *v *= -COLLISION_DAMPING;
    } else if *new_x > high {
        *prev_x = 2.0 * high - *prev_x;
        *new_x = 2.0 * high - *new_x;
        *v *= -COLLISION_DAMPING;
    }
}

pub fn update_colors(
    particles: Query<(&MeshMaterial2d<ColorMaterial>, &ParticleVelocity)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (color, velocity) in &particles {
        materials.get_mut(color).unwrap().color = color::for_velocity(velocity.0.length());
    }
}
