// Copyright 2025 Kensuke Saito
// SPDX-License-Identifier: GPL-2.0-or-later

mod imgui_state;

use crate::{NeedleErr, NeedleError, NeedleLabel, Vertex};
use std::sync::Arc;
use wgpu::{util::DeviceExt, CompositeAlphaMode, Device, Queue, Surface, SurfaceConfiguration};
use winit::{dpi::PhysicalSize, window::Window};

pub use imgui_state::{ImguiMode, ImguiState};

pub struct State<'a> {
    size: PhysicalSize<u32>,
    surface: Surface<'a>,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
}

impl<'a> State<'a> {
    /// Create new needle state from winit Window.
    pub async fn new(window: Arc<Window>) -> NeedleErr<Self> {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        // Surface
        let surface = match instance.create_surface(window) {
            Ok(surface) => Ok(surface),
            Err(e) => Err(NeedleError::FailedToCreateSurface(e)),
        }?;

        // Device and Queue
        let adapters = instance.enumerate_adapters(wgpu::Backends::all());
        let adapter = match adapters
            .iter()
            .find(|adapter| adapter.is_surface_supported(&surface))
        {
            Some(adapter) => Ok(adapter),
            None => Err(NeedleError::FailedToFindValidAdapter),
        }?;
        let (device, queue) = match adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::SPIRV_SHADER_PASSTHROUGH,
                ..Default::default()
            })
            .await
        {
            Ok((device, queue)) => Ok((device, queue)),
            Err(err) => Err(NeedleError::FailedToRequestDevice(err)),
        }?;

        // Config
        let surface_caps = surface.get_capabilities(adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);
        let alpha_mode = surface_caps
            .alpha_modes
            .iter()
            .find(|alpha| **alpha == CompositeAlphaMode::PreMultiplied)
            .copied()
            .unwrap_or(surface_caps.alpha_modes[0]);
        let surface_config = SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        Ok(Self {
            size,
            surface,
            device,
            queue,
            config: surface_config,
        })
    }

    /// Size of window
    #[inline]
    pub const fn size(&self) -> PhysicalSize<u32> {
        self.size
    }

    /// Returns reference to logical wgpu::Device
    #[inline]
    pub const fn device(&self) -> &Device {
        &self.device
    }

    /// Returns reference to wgpu::Queue
    #[inline]
    pub const fn queue(&self) -> &Queue {
        &self.queue
    }

    /// Returns reference to wgpu::SurfaceConfiguration
    #[inline]
    pub const fn surface_config(&self) -> &SurfaceConfiguration {
        &self.config
    }

    /// Resize the window/surface size.
    /// Width and height are passed via `winit::dpi::PhysicalSize`
    pub fn resize(&mut self, size: &PhysicalSize<u32>) {
        if (size.width > 0) && (size.height > 0) {
            self.size = *size;
            self.config.width = size.width;
            self.config.height = size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    /// Render function to call any renderers' render operation using wgpu::CommandEncoder.
    /// After all the renderers have finished rendering, the queue is automatically submitted.
    pub fn render<F>(&mut self, render_func: F) -> NeedleErr<()>
    where
        F: FnOnce(&mut wgpu::CommandEncoder) -> NeedleErr<()>,
    {
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some(&NeedleLabel::CommandEncoder("").to_string()),
            });

        render_func(&mut encoder)?;

        self.queue.submit(std::iter::once(encoder.finish()));

        Ok(())
    }

    /// Get the current texture from Surface.
    /// Upon failure to retrieve surface texture, return the following errors.
    /// - Timeout
    /// - Outdated
    /// - Lost
    /// - OutOfMemory
    /// - Other
    ///
    /// These errors are translated from `wgpu::SurfaceError` into `NeedleError`
    pub fn get_current_texture(&self) -> NeedleErr<wgpu::SurfaceTexture> {
        match self.surface.get_current_texture() {
            Ok(texture) => Ok(texture),
            Err(err) => {
                let err = match err {
                    wgpu::SurfaceError::Timeout => NeedleError::Timeout,
                    wgpu::SurfaceError::Outdated => NeedleError::Outdated,
                    wgpu::SurfaceError::Lost => NeedleError::Lost,
                    wgpu::SurfaceError::OutOfMemory => NeedleError::OutOfMemory,
                    wgpu::SurfaceError::Other => NeedleError::Other,
                };

                Err(err)
            }
        }
    }

    /// Create vertex buffer from vertices.
    /// This requires label for the vertex buffer.
    /// (Can be empty if only a single vertex buffer is used)
    pub fn create_vertex_buffer(&self, label: &str, vertices: &[Vertex]) -> wgpu::Buffer {
        self.device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&NeedleLabel::VertexBuffer(label).to_string()),
                contents: bytemuck::cast_slice(vertices),
                usage: wgpu::BufferUsages::VERTEX,
            })
    }

    /// Create index buffer from indices
    /// This requires label for the index buffer.
    /// (Can be empty if only a single index buffer is used)
    pub fn create_index_buffer(&self, label: &str, indices: &[u16]) -> wgpu::Buffer {
        self.device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&NeedleLabel::IndexBuffer(label).to_string()),
                contents: bytemuck::cast_slice(indices),
                usage: wgpu::BufferUsages::INDEX,
            })
    }
}
