// Constants which the user might want to play with.

use crate::{consts_private::DENSITY_FACTOR, kernel::Kernel};

// Window dimensions in screen space.
pub const WINDOW_SIZE: (u32, u32) = (1600, 900);

// Pixel scale factor.
pub const PIXEL_SIZE: u32 = 4;

// Simulation box dimensions.
pub const BOX_SIZE: (u32, u32) = (1200, 700);

// Simulation box outline constants.
pub const BOX_LINE_WIDTH: f32 = 2.0;

// The number of particles to spawn.
pub const NUM_PARTICLES: usize = 150;
// The radius of each particle on the screen.
pub const PARTICLE_SCREEN_RADIUS: f32 = 7.5;
// The max initial velocity of particles on the screen.
pub const PARTICLE_MAX_INITIAL_V: f32 = 0.001;


// Physical constants.
// How should the velocity be scaled on collision.
pub const COLLISION_DAMPING: f32 = 0.9;
// How strong should gravity be.
pub const GRAVITY_FORCE: f32 = 5.0;
// How far should particle influence go.
pub const SMOOTHING_RADIUS: f32 = 1.0;
// What is the ideal density.
pub const TARGET_DENSITY: f32 = DENSITY_FACTOR;
// How strong should the pressure force be.
pub const PRESSURE_MULTIPLIER: f32 = 10.0;
// Which kernel to use to compute particle influence.
pub const DENSITY_KERNEL: Kernel = Kernel::Spiky2;
