// Copyright 2025 Kensuke Saito
// SPDX-License-Identifier: GPL-2.0-or-later

mod vertex;

pub use vertex::*;

use crate::{utils, NeedleLabel, State};
use wgpu::util::{BufferInitDescriptor, DeviceExt};

#[derive(Debug, Clone)]
pub struct Buffer {
    vertex_buffer: wgpu::Buffer,
    index: u32,
    slot: u32,
    index_buffer: Option<wgpu::Buffer>,
    index_format: Option<wgpu::IndexFormat>,
}

impl Buffer {
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
        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some(&label.to_string()),
            contents: unsafe { utils::data_into_bytes(vertices) },
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_format = indices.map(|_| {
            if size_of::<Uint>() == size_of::<u16>() {
                wgpu::IndexFormat::Uint16
            } else {
                wgpu::IndexFormat::Uint32
            }
        });
        let index_buffer = indices.map(|indices| {
            device.create_buffer_init(&BufferInitDescriptor {
                label: Some(&label.to_string()),
                contents: unsafe { utils::data_into_bytes(indices) },
                usage: wgpu::BufferUsages::INDEX,
            })
        });

        Self {
            vertex_buffer,
            index,
            slot,
            index_buffer,
            index_format,
        }
    }

    #[inline]
    pub fn submit(&self, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_vertex_buffer(self.slot, self.vertex_buffer.slice(..));
        if let (Some(buffer), Some(format)) = (self.index_buffer.as_ref(), self.index_format) {
            render_pass.set_index_buffer(buffer.slice(..), format);
            render_pass.draw_indexed(0..self.index, 0, 0..1);
        } else {
            render_pass.draw(0..self.index, 0..1);
        }
    }

    #[inline]
    pub fn destroy(&self) {
        self.vertex_buffer.destroy();
        if let Some(buffer) = &self.index_buffer {
            buffer.destroy()
        }
    }
}
