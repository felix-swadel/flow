use bevy::prelude::*;

use crate::color;
use crate::consts::*;
use crate::consts_private::*;
use crate::kernel::{self, Kernel};
use crate::physics;
use crate::random;

#[derive(Component)]
pub struct ParticleDensity(pub f32);

#[derive(Component)]
pub struct ParticlePressure(pub f32);

#[derive(Component)]
pub struct ParticlePosition(pub Vec2);

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
        let (x, y) = random::point_in_box(PHYSICAL_HALF_SIZE);
        let v = random::vec_within_disk(PARTICLE_MAX_INITIAL_V);
        println!("Initial v: {}", v);
        commands.spawn((
            // Animation properties.
            Mesh2d(circle.clone()),
            MeshMaterial2d(materials.add(color::for_velocity(v.length()))),
            Transform::from_xyz(x * SCREEN_FACTOR, y * SCREEN_FACTOR, 1.0),
            // Physical properties.
            ParticleDensity(0.0),
            ParticlePressure(0.0),
            ParticlePosition(Vec2 {x, y}),
            ParticleVelocity(v),
            ParticleAcceleration(Vec2::ZERO),
        ));
    }
}

pub fn update_densities_and_pressures(
    mut densities: Query<(Entity, &mut ParticleDensity, &mut ParticlePressure)>,
    positions: Query<&ParticlePosition>,
) {
    // For each particle.
    for (
        entity,
        mut density,
        mut pressure,
    ) in &mut densities {
        // Get the position of that particle.
        let ParticlePosition(sample_point) = positions.get(entity).unwrap();
        // Start with the density at the edge of the container.
        let mut sum = physics::compute_edge_density(sample_point);
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
        pressure.0 = physics::density_to_pressure(density.0);
        println!("Density: {}", density.0);
        println!("Pressure: {}", pressure.0);
    }
}

pub fn update_accelerations(
    mut accelerations: Query<(Entity, &mut ParticleAcceleration)>,
    particles: Query<(&ParticlePosition, &ParticlePressure, &ParticleDensity)>,
) {
    // For each particle.
    for (entity, mut acceleration) in &mut accelerations {
        // Get the position of and density at that particle.
        let (
            ParticlePosition(pos_x),
            _,
            ParticleDensity(density_x),
        ) = particles.get(entity).unwrap();
        // If the only density contribution is from this particle, skip it.
        if (*density_x - DENSITY_FACTOR).abs() < std::f32::EPSILON {
            acceleration.0 = Vec2::ZERO;
            acceleration.0.y -= GRAVITY_FORCE;
            continue;
        }
        // Sum the acceleration contributions of all particles on that position.
        let mut acc = Vec2::ZERO;
        for (
            ParticlePosition(pos_i),
            ParticlePressure(pressure_i),
            ParticleDensity(density_i),
        ) in particles.iter() {
            if std::ptr::addr_eq(pos_x, pos_i) {
                continue;
            }

            acc += pressure_i * kernel::grad_spiky2(pos_x - pos_i) / density_i;
        }
        // Add edge pressure force.
        acc += physics::compute_edge_acceleration(&pos_x, *density_x);
        // Update acceleration.
        acceleration.0 = acc;
        acceleration.0.y -= GRAVITY_FORCE;
        println!("Acc: {}", acceleration.0);
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

pub fn update_positions(
    time: Res<Time>,
    mut particles: Query<(&mut Transform, &mut ParticlePosition, &mut ParticleVelocity)>,
) {
    for (
        mut transform,
        mut position,
        mut velocity
    ) in &mut particles {
        // Integrate particle velocity.
        physics::integrate_velocity(
            &mut position.0.x,
            &mut velocity.0.x,
            time.delta_secs(),
            -PARTICLE_CENTRE_BOUND.0,
            PARTICLE_CENTRE_BOUND.0,
            COLLISION_DAMPING,
        );
        physics::integrate_velocity(
            &mut position.0.y,
            &mut velocity.0.y,
            time.delta_secs(),
            -PARTICLE_CENTRE_BOUND.1,
            PARTICLE_CENTRE_BOUND.1,
            COLLISION_DAMPING,
        );
        println!("Pos: {}", position.0);
        // Propagate position changes to transform to update animation.
        transform.translation.x = position.0.x * SCREEN_FACTOR;
        transform.translation.y = position.0.y * SCREEN_FACTOR;
    }
}

pub fn update_velocities(
    time: Res<Time>,
    mut particles: Query<(&ParticleAcceleration, &mut ParticleVelocity)>,
) {
    for (
        ParticleAcceleration(acceleration),
        mut velocity,
    ) in &mut particles {
        println!("Vel before: {}", velocity.0);
        velocity.0 *= 0.95;
        velocity.0 += time.delta_secs() * acceleration;
        println!("Vel after: {}", velocity.0);
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
