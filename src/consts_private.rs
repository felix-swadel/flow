// Contains constants computed from other constants
// which aren't relevant to the user.

use crate::consts::*;

// Window size constants.
pub const WINDOW_SIZE_F: (f32, f32) = (
    WINDOW_SIZE.0 as f32, WINDOW_SIZE.1 as f32,
);

// Box size constants.
pub const BOX_SIZE_F: (f32, f32) = (
    BOX_SIZE.0 as f32, BOX_SIZE.1 as f32,
);
pub const BOX_HALF_SIZE: (f32, f32) = (
    0.5 * BOX_SIZE_F.0, 0.5 * BOX_SIZE_F.1,
);
pub const BOX_LINE_CENTRE: (f32, f32) = (
    BOX_HALF_SIZE.0 + 0.5 * BOX_LINE_WIDTH,
    BOX_HALF_SIZE.1 + 0.5 * BOX_LINE_WIDTH,
);

// Image size constants.
pub const IMAGE_SIZE: (u32, u32) = (
    BOX_SIZE.0 / PIXEL_SIZE, BOX_SIZE.1 / PIXEL_SIZE,
);

// Scale factor between screen space and physical space.
// We scale down the screen space by a factor of 100
// to get nicer numbers for computing physical properties.
pub const SCREEN_FACTOR: f32 = 100.0;
