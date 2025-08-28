// Copyright 2025 Kensuke Saito
// SPDX-License-Identifier: GPL-2.0-or-later

use crate::{utils, BindGroup, BindGroupLayout, NeedleErr, NeedleLabel};

#[derive(Debug, Clone)]
pub struct Ubo {
    buffer: wgpu::Buffer,
    bind_group: BindGroup,
    slot: u32,
    offset: u64,
}

impl Ubo {
    /// Create new UBO
    pub fn new<T>(
        device: &wgpu::Device,
        label: NeedleLabel,
        layout: &BindGroupLayout,
        slot: u32,
        offset: u64,
    ) -> NeedleErr<Self>
    where
        T: Sized,
    {
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(&label.to_string()),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            size: size_of::<T>() as u64,
            mapped_at_creation: false,
        });
        let bind_group = BindGroup::builder()
            .set_layout(layout)
            .set_buffer(&buffer, 0)
            .build(device, label)?;

        Ok(Self {
            buffer,
            bind_group,
            slot,
            offset,
        })
    }

    /// Update data in UBO
    pub fn update<T>(&mut self, data: &T, queue: &wgpu::Queue)
    where
        T: Sized + Clone,
    {
        let contents = unsafe { utils::data_into_bytes(std::slice::from_ref(data)) }.to_vec();

        queue.write_buffer(&self.buffer, self.offset, &contents);
    }

    /// Submit bind group of UBO to render pass
    #[inline]
    pub fn submit(&self, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_bind_group(self.slot, self.bind_group.bind_group(), &[]);
    }

    /// Destroy buffer
    #[inline]
    pub fn destroy(&self) {
        self.buffer.destroy();
    }
}
