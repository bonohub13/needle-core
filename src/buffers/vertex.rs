// Copyright 2025 Kensuke Saito
// SPDX-License-Identifier: GPL-2.0-or-later

use crate::utils::crop;
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
    pub fn rectangle(size: [f32; 2], offset: [f32; 2], depth: f32, color: &[f32; 4]) -> [Self; 6] {
        let (min_x, min_y) = (
            crop(offset[0], 2.0) - size[0],
            crop(offset[1], 2.0) - size[1],
        );

        [
            Vertex::new([min_x, min_y, depth], *color), // Bottom left
            Vertex::new([min_x, size[1], depth], *color), // Top left
            Vertex::new([size[0], min_y, depth], *color), // Bottom Right
            Vertex::new([size[0], min_y, depth], *color), // Bottom Right
            Vertex::new([min_x, size[1], depth], *color), // Top left
            Vertex::new([size[0], size[1], depth], *color), // Top right
        ]
    }

    /// Create indexed rectangle.
    /// For specification of rectangle, refer to `Vertex::rectangle()`.
    pub fn indexed_rectangle(
        size: [f32; 2],
        offset: [f32; 2],
        depth: f32,
        color: &[f32; 4],
    ) -> ([Self; 4], [u16; 6]) {
        let (min_x, min_y) = (
            crop(offset[0], 2.0) - size[0],
            crop(offset[1], 2.0) - size[1],
        );
        let vertices = [
            Vertex::new([min_x, min_y, depth], *color), // Bottom left
            Vertex::new([size[0], min_y, depth], *color), // Bottom Right
            Vertex::new([size[0], size[1], depth], *color), // Top right
            Vertex::new([min_x, size[1], depth], *color), // Top left
        ];
        let indices =
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

        (vertices, indices)
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
