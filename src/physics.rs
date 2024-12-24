use glam::f32::Vec2;

use crate::consts::{
    DENSITY_KERNEL, PRESSURE_MULTIPLIER, TARGET_DENSITY,
};
use crate::consts_private::DENSITY_FACTOR;
use crate::kernel::{self, Kernel};
use crate::particle::PHYSICAL_HALF_SIZE;

/// Performs a 1D integration of a position [x](f32) moving at a velocity [v](f32)
/// moving in a bounded range from [low](f32) to [high](f32) with velocity damping
/// on collisions.
/// 
/// [x](f32) and [v](f32) are updated in place.
pub fn integrate_velocity(
    x: &mut f32, v: &mut f32, dt: f32,
    low: f32, high: f32, collision_damping: f32,
) {
    let new_x = *x + *v * dt;
    if new_x < low {
        *v *= -collision_damping;
        *x = 2.0 * low - new_x;
    } else if high < new_x {
        *v *= -collision_damping;
        *x = 2.0 * high - new_x;
    } else {
        *x = new_x;
    }
}

pub fn density_to_pressure(density: f32) -> f32 {
    let density_error = density - TARGET_DENSITY;
    density_error * PRESSURE_MULTIPLIER
}

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
    edge_density
}

pub fn compute_edge_acceleration(sample_point: &Vec2, sample_density: f32) -> Vec2 {
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
    println!("Edge displacement: [{}, {}]", edge_displacement.0, edge_displacement.1);
    let edge_pressure = density_to_pressure(DENSITY_FACTOR);
    println!("Edge pressure: {}", edge_pressure);
    let acc = match DENSITY_KERNEL {
        Kernel::Smooth6 => {
            edge_pressure * (
                kernel::grad_smooth6(Vec2 {x: edge_displacement.0, y: 0.0}) +
                kernel::grad_smooth6(Vec2 {x: 0.0, y: edge_displacement.1})
            ) / DENSITY_FACTOR
        },
        Kernel::Spiky2 => {
            edge_pressure * (
                kernel::grad_spiky2(Vec2 {x: edge_displacement.0, y: 0.0}) +
                kernel::grad_spiky2(Vec2 {x: 0.0, y: edge_displacement.1})
            ) / DENSITY_FACTOR
        },
    };
    acc / sample_density
}
