// Copyright 2025 Kensuke Saito
// SPDX-License-Identifier: GPL-2.0-only

mod fonts;

pub use fonts::{FontTypes, Fonts};

pub(crate) fn crop<T: PartialOrd>(val: T, max: T) -> T {
    if val < max {
        val
    } else {
        max
    }
}
