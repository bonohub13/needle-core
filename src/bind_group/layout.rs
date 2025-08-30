// Copyright 2025 Kensuke Saito
// SPDX-License-Identifier: GPL-2.0-or-later

use crate::NeedleLabel;

#[derive(Debug, Clone)]
pub struct BindGroupLayout {
    pub(crate) layout: wgpu::BindGroupLayout,
}

impl BindGroupLayout {
    /// Creates instance of builder for BindGroupLayout
    #[inline]
    pub const fn builder() -> BindGroupLayoutBuilder {
        BindGroupLayoutBuilder { entries: vec![] }
    }

    /// Get inner wgpu::BindGroupLayout
    #[inline]
    pub const fn layout(&self) -> &wgpu::BindGroupLayout {
        &self.layout
    }
}

#[derive(Debug, Clone)]
pub struct BindGroupLayoutBuilder {
    entries: Vec<wgpu::BindGroupLayoutEntry>,
}

impl BindGroupLayoutBuilder {
    /// Add UBO to BindGroupLayout
    pub fn add_ubo(&mut self) -> Self {
        self.entries.push(wgpu::BindGroupLayoutEntry {
            binding: self.entries.len() as u32,
            visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        });

        self.clone()
    }

    /// Build new instance of BindGroupLayout
    pub fn build(&self, device: &wgpu::Device, label: NeedleLabel) -> BindGroupLayout {
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some(&label.to_string()),
            entries: &self.entries,
        });

        BindGroupLayout { layout }
    }
}
