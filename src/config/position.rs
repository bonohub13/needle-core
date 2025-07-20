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

macro_rules! position_impl_from {
    ($type:ty) => {
        impl From<$type> for Position {
            fn from(val: $type) -> Self {
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

        impl From<Position> for $type {
            fn from(element: Position) -> Self {
                match element {
                    Position::TopLeft => 0,
                    Position::Top => 1,
                    Position::TopRight => 2,
                    Position::Left => 3,
                    Position::Center => 4,
                    Position::Right => 5,
                    Position::BottomLeft => 6,
                    Position::Bottom => 7,
                    Position::BottomRight => 8,
                }
            }
        }
    };
}

position_impl_from! { i8 }
position_impl_from! { u8 }
position_impl_from! { i16 }
position_impl_from! { u16 }
position_impl_from! { i32 }
position_impl_from! { u32 }

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

        write!(f, "\"{position}\"")
    }
}
