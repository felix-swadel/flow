use bevy::prelude::{Color, Srgba};
use glam::f32::Vec2;

use crate::const_color_u8;
use crate::consts::{PARTICLE_MAX_INITIAL_V, TARGET_DENSITY};
use crate::consts_private::DENSITY_FACTOR;
use crate::kernel;
use crate::maths::*;
use crate::particle::ParticlePosition;
use crate::physics;

// Background colors.
const COLOR_LOW_PRESSURE: Srgba = bevy::color::palettes::basic::BLUE;
const COLOR_TARGET_PRESSURE: Srgba = bevy::color::palettes::basic::WHITE;
const COLOR_HIGH_PRESSURE: Srgba = bevy::color::palettes::basic::RED;

// Particle colors.
const PARTICLE_COLOR_SLOW: (f32, f32, f32) = const_color_u8!(32, 166, 214);
const PARTICLE_COLOR_FAST: (f32, f32, f32) = const_color_u8!(214, 32, 32);

// We assume that densities range in [0, N * kernel::MAX_VAL].
// That is with influence from 0 particles to influence from up to N particles.
const N: f32 = 5.0;
const MARGIN: f32 = 0.05;
const DENSITY_UPPER_BOUND: f32 = N * DENSITY_FACTOR;
const MARGIN_LOWER_BOUND: f32 = TARGET_DENSITY * (1.0 - MARGIN);
const MARGIN_UPPER_BOUND: f32 =
    TARGET_DENSITY + (DENSITY_UPPER_BOUND - TARGET_DENSITY) * MARGIN;
const UPPER_RANGE: f32 = DENSITY_UPPER_BOUND - MARGIN_UPPER_BOUND;
// We scale the density range to produce nice colours when transformed.
const MAX_SIGMOID_INPUT: f32 = 3.0;

pub fn for_density<'a>(
    sample_point: Vec2,
    particles: impl Iterator<Item = &'a ParticlePosition>,
) -> Color {
    // Compute the density at this point.
    let mut density = physics::compute_edge_density(&sample_point);
    for ParticlePosition(pos_i) in particles {
        let displacement_squared = (sample_point - pos_i).length_squared();
        density += kernel::spiky2(displacement_squared);
    }
    // Color point relative to target density.
    if density < MARGIN_LOWER_BOUND {
        let density_error = MARGIN_LOWER_BOUND - density;
        let error_scaled = density_error * MAX_SIGMOID_INPUT / MARGIN_LOWER_BOUND;
        lerp_color(
            &COLOR_TARGET_PRESSURE,
            &COLOR_LOW_PRESSURE,
            sigmoid(error_scaled),
        )
    } else if density < MARGIN_UPPER_BOUND {
        Color::WHITE
    } else {
        let density_error = density - MARGIN_UPPER_BOUND;
        let scaled_error = density_error * MAX_SIGMOID_INPUT / UPPER_RANGE;
        lerp_color(
            &COLOR_TARGET_PRESSURE,
            &COLOR_HIGH_PRESSURE,
            sigmoid(scaled_error),
        )
    }
}

// Factor to scale the velocity into a nice color range.
pub const PARTICLE_COLOR_FACTOR: f32 = 2.0 / PARTICLE_MAX_INITIAL_V;

pub fn for_velocity(v: f32) -> Color {
    let color_factor = sigmoid(v * PARTICLE_COLOR_FACTOR);
    Color::srgb(
        lerp(PARTICLE_COLOR_SLOW.0, PARTICLE_COLOR_FAST.0, color_factor),
        lerp(PARTICLE_COLOR_SLOW.1, PARTICLE_COLOR_FAST.1, color_factor),
        lerp(PARTICLE_COLOR_SLOW.2, PARTICLE_COLOR_FAST.2, color_factor),
    )
}

fn lerp_color(a: &Srgba, b: &Srgba, t: f32) -> Color {
    Color::srgb(
        lerp(a.red, b.red, t),
        lerp(a.green, b.green, t),
        lerp(a.blue, b.blue, t),
    )
}
