use bevy::prelude::*;
use glam::f32::Vec2;

use crate::consts::{DENSITY_KERNEL, TARGET_DENSITY};
use crate::consts_private::DENSITY_FACTOR;
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
