// Copyright 2025 Kensuke Saito
// SPDX-License-Identifier: GPLv2

pub(crate) fn crop<T: PartialOrd>(val: T, max: T) -> T {
    if val < max {
        val
    } else {
        max
    }
}
