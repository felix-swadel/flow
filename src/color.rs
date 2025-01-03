use bevy::prelude::{Color, Srgba};
use glam::f32::Vec2;

use crate::const_srgba_u8;
use crate::consts::{DENSITY_KERNEL, EDGE_REPULSION, TARGET_DENSITY};
use crate::consts_private::DENSITY_FACTOR;
use crate::maths::*;
use crate::particle::ParticlePosition;
use crate::physics;

// Background colors.
const COLOR_LOW_PRESSURE: Srgba = bevy::color::palettes::basic::BLUE;
const COLOR_TARGET_PRESSURE: Srgba = bevy::color::palettes::basic::WHITE;
const COLOR_HIGH_PRESSURE: Srgba = bevy::color::palettes::basic::RED;

// Particle colors.
const PARTICLE_COLOR_SLOW: Srgba = const_srgba_u8!(32, 166, 214);
const PARTICLE_COLOR_FAST: Srgba = const_srgba_u8!(214, 32, 32);

// We assume that densities range in [0, N * kernel::MAX_VAL].
// That is with influence from 0 particles to influence from up to N particles.
const N: f32 = 5.0;
const MARGIN: f32 = 0.05;
const DENSITY_UPPER_BOUND: f32 = N * DENSITY_FACTOR;
const MARGIN_LOWER_BOUND: f32 = TARGET_DENSITY * (1.0 - MARGIN);
const MARGIN_LOWER_BOUND_INV: f32 = 1.0 / MARGIN_LOWER_BOUND;
const MARGIN_UPPER_BOUND: f32 =
    TARGET_DENSITY + (DENSITY_UPPER_BOUND - TARGET_DENSITY) * MARGIN;
const UPPER_DENSITY_RANGE: f32 = DENSITY_UPPER_BOUND - MARGIN_UPPER_BOUND;
const UPPER_DENSITY_RANGE_INV: f32 = 1.0 / UPPER_DENSITY_RANGE;

pub fn for_density<'a>(
    sample_point: Vec2,
    particles: impl Iterator<Item = &'a ParticlePosition>,
) -> Color {
    // Compute the density at this point.
    let mut density = if EDGE_REPULSION {
        physics::compute_edge_density(&sample_point)
    } else {
        0.0
    };
    for ParticlePosition(pos_i) in particles {
        let displacement_squared = (sample_point - pos_i).length_squared();
        density += DENSITY_KERNEL.influence(displacement_squared);
    }
    // Color point relative to target density.
    if density < MARGIN_LOWER_BOUND {
        lerp_color(
            &COLOR_LOW_PRESSURE,
            &COLOR_TARGET_PRESSURE,
            density * MARGIN_LOWER_BOUND_INV,
        )
    } else if density < MARGIN_UPPER_BOUND {
        Color::WHITE
    } else {
        let density_error = density - MARGIN_UPPER_BOUND;
        lerp_color(
            &COLOR_TARGET_PRESSURE,
            &COLOR_HIGH_PRESSURE,
            1.0_f32.min(density_error * UPPER_DENSITY_RANGE_INV),
        )
    }
}

pub fn for_velocity(v: f32) -> Color {
    // Assume [0, 2] is a reasonable range for velocity values.
    let color_factor = 1.0_f32.min(v * 0.5);
    lerp_color(&PARTICLE_COLOR_SLOW, &PARTICLE_COLOR_FAST, color_factor)
}

fn lerp_color(a: &Srgba, b: &Srgba, t: f32) -> Color {
    Color::srgb(
        lerp(a.red, b.red, t),
        lerp(a.green, b.green, t),
        lerp(a.blue, b.blue, t),
    )
}
