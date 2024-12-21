#[macro_export]
macro_rules! u8_to_f32 {
    ($r:literal) => {
        $r as f32 / 255.0
    };
}

#[macro_export]
macro_rules! const_color_u8 {
    ($r:literal, $g:literal, $b:literal) => {
        (
            crate::u8_to_f32!($r),
            crate::u8_to_f32!($g),
            crate::u8_to_f32!($b),
        )
    };
}