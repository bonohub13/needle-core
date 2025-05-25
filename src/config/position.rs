// Copyright 2025 Kensuke Saito
// SPDX-License-Identifier: GPL-2.0-only

use serde::Deserialize;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub enum Position {
    Center,
    Top,
    Bottom,
    Right,
    Left,
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
}

impl From<i32> for Position {
    fn from(val: i32) -> Self {
        match val {
            0 => Self::TopLeft,
            1 => Self::Top,
            2 => Self::TopRight,
            3 => Self::Left,
            4 => Self::Center,
            5 => Self::Right,
            6 => Self::BottomLeft,
            7 => Self::Bottom,
            8 => Self::BottomRight,
            _ => Self::Center,
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let position = match self {
            Self::Center => "Center",
            Self::Top => "Top",
            Self::Bottom => "Bottom",
            Self::Right => "Right",
            Self::Left => "Left",
            Self::TopRight => "TopRight",
            Self::TopLeft => "TopLeft",
            Self::BottomRight => "BottomRight",
            Self::BottomLeft => "BottomLeft",
        };

        write!(f, "\"{}\"", position)
    }
}
