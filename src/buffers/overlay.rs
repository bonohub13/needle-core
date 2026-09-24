// Copyright 2025 Kensuke Saito
// SPDX-License-Identifier: GPL-2.0-or-later

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct OverlayInfo {
    top_left: glm::Vec2,
    size: glm::Vec2,
    color: glm::Vec4,
}

impl OverlayInfo {
    const COORD_MAX: glm::Vec2 = glm::vec2(2.0, 2.0);
    const COORD_MIN: glm::Vec2 = glm::vec2(0.0, 0.0);
    const COLOR_MAX: glm::Vec4 = glm::vec4(1.0, 1.0, 1.0, 1.0);
    const COLOR_MIN: glm::Vec4 = glm::vec4(0.0, 0.0, 0.0, 0.0);

    #[inline]
    pub const fn new(top_left: [f32; 2], size: [f32; 2], color: [f32; 4]) -> Self {
        Self {
            top_left: glm::vec2(
                top_left[0].min(Self::COORD_MAX.x).max(Self::COORD_MIN.x),
                top_left[1].min(Self::COORD_MAX.y).max(Self::COORD_MIN.y),
            ),
            size: glm::vec2(
                size[0].min(Self::COORD_MAX.x).max(Self::COORD_MIN.x),
                size[1].min(Self::COORD_MAX.y).max(Self::COORD_MIN.y),
            ),
            color: glm::vec4(
                color[0].min(Self::COLOR_MAX.x).max(Self::COLOR_MIN.x),
                color[1].min(Self::COLOR_MAX.y).max(Self::COLOR_MIN.y),
                color[2].min(Self::COLOR_MAX.z).max(Self::COLOR_MIN.z),
                color[3].min(Self::COLOR_MAX.w).max(Self::COLOR_MIN.w),
            ),
        }
    }

    #[inline]
    pub const fn top_left(&self) -> &glm::Vec2 {
        &self.top_left
    }

    #[inline]
    pub const fn size(&self) -> &glm::Vec2 {
        &self.size
    }

    #[inline]
    pub const fn color(&self) -> &glm::Vec4 {
        &self.color
    }

    #[inline]
    pub const fn set_top_left(&mut self, top_left: [f32; 2]) {
        self.top_left = glm::vec2(
            top_left[0].min(Self::COORD_MAX.x).max(Self::COORD_MIN.x),
            top_left[1].min(Self::COORD_MAX.y).max(Self::COORD_MIN.y),
        );
    }

    #[inline]
    pub const fn set_size(&mut self, size: [f32; 2]) {
        self.size = glm::vec2(
            size[0].min(Self::COORD_MAX.x).max(Self::COORD_MIN.x),
            size[1].min(Self::COORD_MAX.y).max(Self::COORD_MIN.y),
        );
    }

    #[inline]
    pub const fn set_color(&mut self, color: [f32; 4]) {
        self.color = glm::vec4(
            color[0].min(Self::COLOR_MAX.x).max(Self::COLOR_MIN.x),
            color[1].min(Self::COLOR_MAX.y).max(Self::COLOR_MIN.y),
            color[2].min(Self::COLOR_MAX.z).max(Self::COLOR_MIN.z),
            color[3].min(Self::COLOR_MAX.w).max(Self::COLOR_MIN.w),
        );
    }
}
