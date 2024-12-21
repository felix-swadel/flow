// Constants which the user might want to play with.

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
pub const PARTICLE_MAX_INITIAL_V: f32 = 0.002;


// Physical constants.
// How should the velocity be scaled on collision.
pub const COLLISION_DAMPING: f32 = 0.7;
// How strong should gravity be.
pub const GRAVITY_FORCE: f32 = 0.0;
// How far should particle influence go.
pub const SMOOTHING_RADIUS: f32 = 1.25;
// What is the ideal density.
pub const TARGET_DENSITY: f32 = 1.0;
// How strong should the pressure force be.
pub const PRESSURE_MULTIPLIER: f32 = 1.0;
