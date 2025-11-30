// Copyright 2025 Kensuke Saito
// SPDX-License-Identifier: GPL-2.0-or-later

use std::mem::size_of;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    position: glm::Vec3,
    color: glm::Vec4,
}

impl Vertex {
    const VERTEX_ATTR: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x4];
    const VERTEX_COORD_MAX: glm::Vec2 = glm::vec2(2.0, 2.0);
    const VERTEX_COORD_MIN: glm::Vec2 = glm::vec2(-1.0, -1.0);

    /// Create a new instance of Vertex
    #[inline]
    pub const fn new(position: [f32; 3], color: [f32; 4]) -> Self {
        Self {
            position: glm::vec3(position[0], position[1], position[2]),
            color: glm::vec4(color[0], color[1], color[2], color[3]),
        }
    }

    /// Create a Vertex of rectangle with the specifications below.
    /// - size: width, height
    /// - offset: x, y offset
    /// - depth: z axis offset
    /// - color: RGBA for all vertex
    pub const fn rectangle(
        size: [f32; 2],
        offset: [f32; 2],
        depth: f32,
        color: &[f32; 4],
    ) -> [Self; 6] {
        let min = glm::vec2(
            (offset[0].min(Self::VERTEX_COORD_MAX.x) - size[0]).max(Self::VERTEX_COORD_MIN.x),
            (offset[1].min(Self::VERTEX_COORD_MAX.y) - size[1]).max(Self::VERTEX_COORD_MIN.y),
        );
        let max = glm::vec2(
            (size[0] + offset[0]).min(Self::VERTEX_COORD_MAX.x),
            (size[1] + offset[1]).min(Self::VERTEX_COORD_MAX.y),
        );

        [
            Vertex::new([min.x, min.y, depth], *color), // Bottom left
            Vertex::new([min.x, max.y, depth], *color), // Top left
            Vertex::new([max.x, min.y, depth], *color), // Bottom Right
            Vertex::new([max.x, min.y, depth], *color), // Bottom Right
            Vertex::new([min.x, max.y, depth], *color), // Top left
            Vertex::new([max.x, max.y, depth], *color), // Top right
        ]
    }

    /// Create indexed rectangle.
    /// For specification of rectangle, refer to `Vertex::rectangle()`.
    pub const fn indexed_rectangle(
        size: [f32; 2],
        offset: [f32; 2],
        depth: f32,
        color: &[f32; 4],
    ) -> ([Self; 4], [u16; 6]) {
        const INDICES: [u16; 6] =
            /* Order to draw
             * Bottom left
             * Top left
             * Bottom right
             * ---
             * Bottom right
             * Top left
             * Top right
             */
            [
                0, 3, 1, // Lower left triangle
                1, 3, 2, // Upper right triangle
            ];

        let min = glm::vec2(
            (offset[0].min(Self::VERTEX_COORD_MAX.x) - size[0]).max(Self::VERTEX_COORD_MIN.x),
            (offset[1].min(Self::VERTEX_COORD_MAX.y) - size[1]).max(Self::VERTEX_COORD_MIN.y),
        );
        let max = glm::vec2(
            (size[0] + offset[0]).min(Self::VERTEX_COORD_MAX.x),
            (size[1] + offset[1]).min(Self::VERTEX_COORD_MAX.y),
        );
        let vertices = [
            Vertex::new([min.x, min.y, depth], *color), // Bottom left
            Vertex::new([max.x, min.y, depth], *color), // Bottom Right
            Vertex::new([max.x, max.y, depth], *color), // Top right
            Vertex::new([min.x, max.y, depth], *color), // Top left
        ];

        (vertices, INDICES)
    }

    /// Create buffer layout for Vertex
    pub const fn buffer_layout<'layout>() -> wgpu::VertexBufferLayout<'layout> {
        wgpu::VertexBufferLayout {
            array_stride: size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::VERTEX_ATTR,
        }
    }
}
