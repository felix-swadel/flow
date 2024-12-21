use bevy::prelude::*;

#[derive(Component)]
pub struct FrameRateUI;

#[derive(Resource)]
pub struct FrameRateLastUpdate(pub f32);

// Frame rate UI constants.
pub const FRAME_RATE_FONT_SIZE: f32 = 33.0;
pub const FRAME_RATE_TEXT_PADDING: Val = Val::Px(5.0);
pub const FRAME_RATE_UPDATE_INTERVAL: f32 = 0.2;

pub fn update_frame_rate(
    time: Res<Time>,
    mut last_update: ResMut<FrameRateLastUpdate>,
    score_root: Single<Entity, (With<FrameRateUI>, With<Text>)>,
    mut writer: TextUiWriter,
) {
    if time.elapsed_secs() - last_update.0 > FRAME_RATE_UPDATE_INTERVAL {
        last_update.0 += FRAME_RATE_UPDATE_INTERVAL;
        let frame_rate = 1.0 / time.delta_secs();
        *writer.text(*score_root, 1) = frame_rate.to_string();
    }
}
