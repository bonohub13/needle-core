// Copyright 2025 Kensuke Saito
// SPDX-License-Identifier: GPL-2.0-or-later

mod overlay;
mod ubo;
mod vertex;

pub use overlay::*;
pub use ubo::*;
pub use vertex::*;

use crate::{utils, NeedleLabel, State};
use wgpu::util::{BufferInitDescriptor, DeviceExt};

#[derive(Debug, Clone)]
pub struct Buffer {
    buffer: wgpu::Buffer,
    index: u32,
    slot: u32,
    offset: u64,
    index_format: Option<wgpu::IndexFormat>,
}

impl Buffer {
    /// Create new Buffer
    /// - Optional: indices for vertex
    pub fn new<Uint>(
        state: &State,
        label: NeedleLabel,
        vertices: &[Vertex],
        slot: u32,
        indices: Option<&[Uint]>,
    ) -> Self
    where
        Uint: Sized,
    {
        let device = state.device();
        let index = if let Some(indices) = indices {
            indices.len()
        } else {
            vertices.len()
        } as u32;
        let (buffer, offset) = match indices {
            Some(indices) => {
                let (contents, offset): (Vec<u8>, u64) = {
                    let vertices = unsafe { utils::data_into_bytes(vertices) };
                    let indices = unsafe { utils::data_into_bytes(indices) };

                    ([vertices, indices].concat(), vertices.len() as u64)
                };
                let buffer = device.create_buffer_init(&BufferInitDescriptor {
                    label: Some(&label.to_string()),
                    contents: &contents,
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::INDEX,
                });

                (buffer, offset)
            }
            None => {
                let contents = unsafe { utils::data_into_bytes(vertices) };
                let buffer = device.create_buffer_init(&BufferInitDescriptor {
                    label: Some(&label.to_string()),
                    contents,
                    usage: wgpu::BufferUsages::VERTEX,
                });

                (buffer, contents.len() as u64)
            }
        };
        let index_format = indices.map(|_| {
            if size_of::<Uint>() == size_of::<u16>() {
                wgpu::IndexFormat::Uint16
            } else {
                wgpu::IndexFormat::Uint32
            }
        });

        Self {
            buffer,
            index,
            slot,
            offset,
            index_format,
        }
    }

    /// Submit buffer to render pass
    #[inline]
    pub fn submit(&self, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_vertex_buffer(self.slot, self.buffer.slice(..self.offset));
        if let Some(format) = self.index_format {
            render_pass.set_index_buffer(self.buffer.slice(self.offset..), format);
        }
    }

    /// Draw contents of buffer via render pass
    #[inline]
    pub fn draw(&self, render_pass: &mut wgpu::RenderPass) {
        if self.index_format.is_some() {
            render_pass.draw_indexed(0..self.index, 0, 0..1);
        } else {
            render_pass.draw(0..self.index, 0..1);
        }
    }

    /// Destroy buffer
    #[inline]
    pub fn destroy(&self) {
        self.buffer.destroy();
    }
}
