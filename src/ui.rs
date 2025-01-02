use bevy::prelude::*;

#[derive(Component)]
pub struct UI;

#[derive(Resource)]
pub struct UILastUpdate(pub f32);

#[derive(Resource)]
pub struct AverageEK(pub f32);

// Frame rate UI constants.
pub const FRAME_RATE_FONT_SIZE: f32 = 33.0;
pub const FRAME_RATE_TEXT_PADDING: Val = Val::Px(5.0);
pub const FRAME_RATE_UPDATE_INTERVAL: f32 = 0.2;

pub fn update(
    time: Res<Time>,
    ek: Res<AverageEK>,
    mut last_update: ResMut<UILastUpdate>,
    score_root: Single<Entity, (With<UI>, With<Text>)>,
    mut writer: TextUiWriter,
) {
    if time.elapsed_secs() - last_update.0 > FRAME_RATE_UPDATE_INTERVAL {
        last_update.0 += FRAME_RATE_UPDATE_INTERVAL;
        let frame_rate = 1.0 / time.delta_secs();
        *writer.text(*score_root, 1) = format!("{:>5.2}", frame_rate);
        *writer.text(*score_root, 3) = format!("{:>4.2}", ek.0);
    }
}
