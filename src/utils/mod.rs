// Copyright 2025 Kensuke Saito
// SPDX-License-Identifier: GPL-2.0-or-later

mod fonts;

pub use fonts::{Font, FontType, FontTypes, Fonts};

/// Crop value to maximum value
pub(crate) fn crop<T: PartialOrd>(val: T, max: T) -> T {
    if val < max {
        val
    } else {
        max
    }
}

/// Parse datas into bytes
pub(crate) unsafe fn data_into_bytes<T>(data: &[T]) -> &[u8]
where
    T: Sized,
{
    let p_data = &data[0] as *const T;

    unsafe { std::slice::from_raw_parts(p_data as *const u8, size_of_val(data)) }
}
