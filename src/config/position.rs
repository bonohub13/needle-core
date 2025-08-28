// Copyright 2025 Kensuke Saito
// SPDX-License-Identifier: GPL-2.0-or-later

use serde::Deserialize;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub enum Position {
    Center,
    Top,
    Bottom,
    Left,
    Right,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Position {
    pub const CENTER: i8 = 0;
    pub const TOP: i8 = 1;
    pub const BOTTOM: i8 = 2;
    pub const LEFT: i8 = 3;
    pub const RIGHT: i8 = 4;
    pub const TOP_LEFT: i8 = 5;
    pub const TOP_RIGHT: i8 = 6;
    pub const BOTTOM_LEFT: i8 = 7;
    pub const BOTTOM_RIGHT: i8 = 8;
    pub const MAX: i8 = Self::BOTTOM_RIGHT;
}

macro_rules! position_impl_from {
    ($type:ty) => {
        impl From<$type> for Position {
            fn from(val: $type) -> Self {
                const CENTER: $type = Position::CENTER as $type;
                const TOP: $type = Position::TOP as $type;
                const BOTTOM: $type = Position::BOTTOM as $type;
                const LEFT: $type = Position::LEFT as $type;
                const RIGHT: $type = Position::RIGHT as $type;
                const TOP_LEFT: $type = Position::TOP_LEFT as $type;
                const TOP_RIGHT: $type = Position::TOP_RIGHT as $type;
                const BOTTOM_LEFT: $type = Position::BOTTOM_LEFT as $type;
                const BOTTOM_RIGHT: $type = Position::BOTTOM_RIGHT as $type;

                match val {
                    CENTER => Self::Center,
                    TOP => Self::Top,
                    BOTTOM => Self::Bottom,
                    LEFT => Self::Left,
                    RIGHT => Self::Right,
                    TOP_LEFT => Self::TopLeft,
                    TOP_RIGHT => Self::TopRight,
                    BOTTOM_LEFT => Self::BottomLeft,
                    BOTTOM_RIGHT => Self::BottomRight,
                    _ => Self::Center,
                }
            }
        }

        impl From<Position> for $type {
            fn from(element: Position) -> Self {
                const CENTER: $type = Position::CENTER as $type;
                const TOP: $type = Position::TOP as $type;
                const BOTTOM: $type = Position::BOTTOM as $type;
                const LEFT: $type = Position::LEFT as $type;
                const RIGHT: $type = Position::RIGHT as $type;
                const TOP_LEFT: $type = Position::TOP_LEFT as $type;
                const TOP_RIGHT: $type = Position::TOP_RIGHT as $type;
                const BOTTOM_LEFT: $type = Position::BOTTOM_LEFT as $type;
                const BOTTOM_RIGHT: $type = Position::BOTTOM_RIGHT as $type;

                match element {
                    Position::Center => CENTER.into(),
                    Position::Top => TOP,
                    Position::Bottom => BOTTOM,
                    Position::Left => LEFT,
                    Position::Right => RIGHT,
                    Position::TopLeft => TOP_LEFT,
                    Position::TopRight => TOP_RIGHT,
                    Position::BottomLeft => BOTTOM_LEFT,
                    Position::BottomRight => BOTTOM_RIGHT,
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
