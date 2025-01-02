use bevy::prelude::*;
use glam::f32::Vec2;

use crate::consts::{COLLISION_DAMPING, DENSITY_KERNEL, PARTICLE_SCREEN_RADIUS, TARGET_DENSITY};
use crate::consts_private::{DENSITY_FACTOR, SCREEN_FACTOR};
use crate::kernel::{self, Kernel};
use crate::maths::sigmoid;
use crate::particle::PHYSICAL_HALF_SIZE;

#[derive(Resource)]
pub struct StartupDamping(pub f32);

pub fn update_startup_damping(
    time: Res<Time>,
    mut damping: ResMut<StartupDamping>,
) {
    damping.0 = sigmoid(0.25 * time.elapsed_secs());
}

pub fn density_to_pressure(density: f32, pressure_multiplier: f32) -> f32 {
    let density_error = density - TARGET_DENSITY;
    density_error * pressure_multiplier
}

const EDGE_DENSITY: f32 = 1.2 * TARGET_DENSITY;
const EDGE_DENSITY_FACTOR: f32 = EDGE_DENSITY / DENSITY_FACTOR;

pub fn compute_edge_density(sample_point: &Vec2) -> f32 {
    let edge_displacement = (
        PHYSICAL_HALF_SIZE.0 - sample_point.x.abs(),
        PHYSICAL_HALF_SIZE.1 - sample_point.y.abs(),
    );
    let displacement_squared = (
        edge_displacement.0 * edge_displacement.0,
        edge_displacement.1 * edge_displacement.1,
    );
    let edge_density = match DENSITY_KERNEL {
        Kernel::Smooth6 =>
            kernel::smooth6(displacement_squared.0) +
            kernel::smooth6(displacement_squared.1),
        Kernel::Spiky2 =>
            kernel::spiky2(displacement_squared.0) +
            kernel::spiky2(displacement_squared.1),
    };
    edge_density * EDGE_DENSITY_FACTOR
}

pub fn compute_edge_acceleration(
    sample_point: &Vec2, sample_density: f32, pressure_multiplier: f32,
) -> Vec2 {
    let edge_displacement = (
        sample_point.x - if sample_point.x > 0.0 {
            PHYSICAL_HALF_SIZE.0
        } else {
            -PHYSICAL_HALF_SIZE.0
        },
        sample_point.y - if sample_point.y > 0.0 {
            PHYSICAL_HALF_SIZE.1
        } else {
            -PHYSICAL_HALF_SIZE.1
        },
    );
    let edge_pressure = density_to_pressure(EDGE_DENSITY, pressure_multiplier);
    let acc = match DENSITY_KERNEL {
        Kernel::Smooth6 => {
            edge_pressure * (
                kernel::grad_smooth6(Vec2 {x: edge_displacement.0, y: 0.0}) +
                kernel::grad_smooth6(Vec2 {x: 0.0, y: edge_displacement.1})
            ) / EDGE_DENSITY
        },
        Kernel::Spiky2 => {
            edge_pressure * (
                kernel::grad_spiky2(Vec2 {x: edge_displacement.0, y: 0.0}) +
                kernel::grad_spiky2(Vec2 {x: 0.0, y: edge_displacement.1})
            ) / EDGE_DENSITY
        },
    };
    acc / sample_density
}

pub struct VerletResult {
    pub prev_x: Vec2,
    pub x: Vec2,
    pub v: Vec2,
    pub moved: bool,
}

pub const PARTICLE_RADIUS: f32 = PARTICLE_SCREEN_RADIUS / SCREEN_FACTOR;
const PARTICLE_CENTRE_BOUND: (f32, f32) = (
    PHYSICAL_HALF_SIZE.0 - PARTICLE_RADIUS, PHYSICAL_HALF_SIZE.1 - PARTICLE_RADIUS,
);

/// Computes the verlet integrated next position and velocity of a particle
/// based on its acceleration and previous and current positions.
/// In the event of a boundary collision, adjusts the previous position,
/// new position, and velocity.
pub fn verlet(
    prev_x: &Option<Vec2>,
    x: &Vec2,
    v: &Vec2,
    a: &Vec2,
    dt: f32,
) -> VerletResult {
    let delta_x = match prev_x {
        None => {
            // N = 1 integration case.
            *v * dt + 0.5 * a * dt * dt
        },
        Some(prev_x) => {
            *x - prev_x + a * dt * dt
        },
    };
    if delta_x.length() < std::f32::EPSILON {
        return VerletResult {
            prev_x: *x,
            x: *x,
            v: Vec2::ZERO,
            moved: false,
        }
    }

    let mut curr_x = *x;
    let mut next_x = *x + delta_x;
    let mut next_v = delta_x / dt;

    boundary_check(
        &mut curr_x.x,
        &mut next_x.x,
        &mut next_v.x,
        -PARTICLE_CENTRE_BOUND.0, PARTICLE_CENTRE_BOUND.0,
    );
    boundary_check(
        &mut curr_x.y,
        &mut next_x.y,
        &mut next_v.y,
        -PARTICLE_CENTRE_BOUND.1, PARTICLE_CENTRE_BOUND.1,
    );

    VerletResult {
        prev_x: curr_x,
        x: next_x,
        v: next_v,
        moved: true,
    }
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
