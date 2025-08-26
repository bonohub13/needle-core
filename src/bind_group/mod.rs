// Copyright 2025 Kensuke Saito
// SPDX-License-Identifier: GPL-2.0-or-later

mod layout;

pub use layout::*;

use crate::{NeedleErr, NeedleError, NeedleLabel};

#[derive(Debug, Clone)]
pub struct BindGroup {
    pub(crate) bind_group: wgpu::BindGroup,
}

impl BindGroup {
    #[inline]
    pub const fn builder() -> BindGroupBuilder<'static> {
        BindGroupBuilder {
            entries: vec![],
            layout: None,
        }
    }

    #[inline]
    pub const fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }
}

#[derive(Debug, Clone)]
pub struct BindGroupBuilder<'a> {
    entries: Vec<wgpu::BindGroupEntry<'a>>,
    layout: Option<&'a BindGroupLayout>,
}

impl<'a> BindGroupBuilder<'a> {
    #[inline]
    pub fn set_layout(&mut self, layout: &'a BindGroupLayout) -> Self {
        self.layout = Some(layout);

        self.clone()
    }

    #[inline]
    pub fn set_buffer(&mut self, buffer: &'a wgpu::Buffer, offset: u64) -> Self {
        self.entries.push(wgpu::BindGroupEntry {
            binding: self.entries.len() as u32,
            resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                buffer,
                offset,
                size: None,
            }),
        });

        self.clone()
    }

    pub fn build(&self, device: &wgpu::Device, label: NeedleLabel) -> NeedleErr<BindGroup> {
        let bind_group = match self.layout {
            Some(layout) => Ok(device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some(&label.to_string()),
                layout: layout.layout(),
                entries: &self.entries,
            })),
            None => Err(NeedleError::InvalidBindingBufferLayout),
        }?;

        Ok(BindGroup { bind_group })
    }
}
