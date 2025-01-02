use bevy::prelude::*;

use crate::physics::StartupDamping;

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
    damping: Res<StartupDamping>,
    mut last_update: ResMut<UILastUpdate>,
    ui_root: Single<Entity, (With<UI>, With<Text>)>,
    mut writer: TextUiWriter,
) {
    if time.elapsed_secs() - last_update.0 > FRAME_RATE_UPDATE_INTERVAL {
        last_update.0 += FRAME_RATE_UPDATE_INTERVAL;
        let frame_rate = 1.0 / time.delta_secs();
        *writer.text(*ui_root, 1) = format!("{:>5.2}", frame_rate);
        *writer.text(*ui_root, 3) = format!("{:>4.2}", ek.0);
        *writer.text(*ui_root, 5) = format!("{:>4.2}", damping.0);
    }
}
