#[macro_export]
macro_rules! u8_to_f32 {
    ($r:literal) => {
        $r as f32 / 255.0
    };
}

#[macro_export]
macro_rules! const_srgba_u8 {
    ($r:literal, $g:literal, $b:literal) => {
        Srgba {
            red: crate::u8_to_f32!($r),
            green: crate::u8_to_f32!($g),
            blue: crate::u8_to_f32!($b),
            alpha: 1.0,
        }
    };
}