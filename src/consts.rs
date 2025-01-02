// Constants which the user might want to play with.

use crate::kernel::Kernel;

// Window dimensions in screen space.
pub const WINDOW_SIZE: (u32, u32) = (1600, 900);

// Pixel scale factor.
pub const PIXEL_SIZE: u32 = 4;

// Simulation box dimensions.
pub const BOX_SIZE: (u32, u32) = (1200, 700);

// Simulation box outline constants.
pub const BOX_LINE_WIDTH: f32 = 2.0;

// The number of particles to spawn.
pub const NUM_PARTICLES: usize = 200;
// The radius of each particle on the screen.
pub const PARTICLE_SCREEN_RADIUS: f32 = 5.0;


// Physical constants.
// How should the velocity be scaled on collision.
pub const COLLISION_DAMPING: f32 = 1.0;
// Should particles be repelled from the edge of the box.
pub const EDGE_REPULSION: bool = false;
// Should we apply startup damping.
pub const STARTUP_DAMPING: bool = true;
// How long should we take to ramp up to full pressure in seconds.
pub const STARTUP_DAMPING_INTERVAL: f32 = 2.0;
// How strong should gravity be.
pub const GRAVITY_FORCE: f32 = 0.0;
// How far should particle influence go.
pub const SMOOTHING_RADIUS: f32 = 1.2;
// What is the ideal density.
pub const TARGET_DENSITY: f32 = 2.75;
// How strong should the pressure force be.
pub const PRESSURE_MULTIPLIER: f32 = 65.0;
// Which kernel to use to compute particle influence.
pub const DENSITY_KERNEL: Kernel = Kernel::Spiky2;
