// Copyright 2025 Kensuke Saito
// SPDX-License-Identifier: GPL-2.0-or-later

use crate::{Buffer, NeedleErr, NeedleError, NeedleLabel, State};
use std::{
    fs::OpenOptions,
    io::Read,
    path::{Path, PathBuf},
};
use wgpu::{Device, Queue, RenderPass, RenderPipeline, SurfaceConfiguration};
use winit::dpi::PhysicalSize;

pub struct ShaderRenderer {
    buffer: Buffer,
    pipeline: RenderPipeline,
}

#[derive(Debug)]
pub struct ShaderRendererDescriptor<'desc> {
    /// Path to vertex shader
    pub vert_shader_path: PathBuf,
    /// Path to fragment shader
    pub frag_shader_path: PathBuf,
    /// Buffer
    pub buffer: Buffer,
    /// Buffer layouts of Vertex buffer
    pub vertex_buffer_layouts: wgpu::VertexBufferLayout<'desc>,
    /// Depth Stencil
    pub depth_stencil: Option<wgpu::DepthStencilState>,
    /// Label used for vertex buffer and index buffer
    pub label: Option<&'desc str>,
}

impl ShaderRenderer {
    /// Creates new instance of ShaderRenderer
    /// Vertex buffer must be passed, however index buffer is optional.
    /// For further specifications, refer to ShaderRendererDescriptor.
    pub fn new(state: &State, desc: &ShaderRendererDescriptor) -> NeedleErr<Self> {
        let label = match desc.label {
            Some(label) => label.to_string(),
            None => "Render".to_string(),
        };
        let vert_shader_code = Self::read_shader(&desc.vert_shader_path)?;
        let frag_shader_code = Self::read_shader(&desc.frag_shader_path)?;
        let vert_shader = unsafe {
            state.device().create_shader_module_passthrough(
                wgpu::ShaderModuleDescriptorPassthrough::SpirV(wgpu::ShaderModuleDescriptorSpirV {
                    label: Some(&NeedleLabel::Shader("Vertex").to_string()),
                    source: wgpu::util::make_spirv_raw(&vert_shader_code),
                }),
            )
        };
        let frag_shader = unsafe {
            state.device().create_shader_module_passthrough(
                wgpu::ShaderModuleDescriptorPassthrough::SpirV(wgpu::ShaderModuleDescriptorSpirV {
                    label: Some(&NeedleLabel::Shader("Fragment").to_string()),
                    source: wgpu::util::make_spirv_raw(&frag_shader_code),
                }),
            )
        };
        let render_pipeline_layout =
            state
                .device()
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some(&NeedleLabel::PipelineLayout(&label).to_string()),
                    bind_group_layouts: &[],
                    push_constant_ranges: &[],
                });
        let render_pipeline =
            state
                .device()
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some(&NeedleLabel::Pipeline(&label).to_string()),
                    layout: Some(&render_pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &vert_shader,
                        entry_point: Some("main"),
                        buffers: &[desc.vertex_buffer_layouts.clone()],
                        compilation_options: wgpu::PipelineCompilationOptions::default(),
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &frag_shader,
                        entry_point: Some("main"),
                        compilation_options: wgpu::PipelineCompilationOptions::default(),
                        targets: &[Some(wgpu::ColorTargetState {
                            format: state.surface_config().format,
                            blend: Some(wgpu::BlendState::REPLACE),
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                    }),
                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::TriangleList,
                        strip_index_format: None,
                        front_face: wgpu::FrontFace::Ccw,
                        cull_mode: Some(wgpu::Face::Back),
                        unclipped_depth: false,
                        polygon_mode: wgpu::PolygonMode::Fill,
                        conservative: false,
                    },
                    depth_stencil: desc.depth_stencil.clone(),
                    multisample: wgpu::MultisampleState {
                        count: 1,
                        mask: !0,
                        alpha_to_coverage_enabled: false,
                    },
                    multiview: None,
                    cache: None,
                });

        Ok(Self {
            buffer: desc.buffer.clone(),
            pipeline: render_pipeline,
        })
    }

    /// Returns reference to render pipeline.
    #[inline]
    pub const fn pipeline(&self) -> &RenderPipeline {
        &self.pipeline
    }

    /// Returns reference to vertex buffer.
    #[inline]
    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    /// Overwrites all vertex buffers.
    /// Pre-existing vertex buffers are all destroyed.
    pub fn set_buffer(&mut self, buffer: Buffer) -> NeedleErr<()> {
        self.buffer.destroy();
        self.buffer = buffer;

        Ok(())
    }

    fn read_shader(path: &Path) -> NeedleErr<Box<[u8]>> {
        let mut reader = match OpenOptions::new().read(true).open(path) {
            Ok(file) => Ok(file),
            Err(err) => Err(NeedleError::FailedToReadShader(err.into())),
        }?;
        let mut buffer = vec![];

        match reader.read_to_end(&mut buffer) {
            Ok(_) => Ok(()),
            Err(err) => Err(NeedleError::FailedToReadShader(err.into())),
        }?;
        if (buffer.len() & 4) != 0 {
            buffer.extend(std::iter::repeat_n(0, buffer.len() % 4));
        }

        let buffer = Box::from_iter(buffer);

        Ok(buffer)
    }
}

impl super::Renderer for ShaderRenderer {
    fn resize(&mut self, _size: &PhysicalSize<u32>) {}

    fn update(&mut self, _queue: &Queue, _config: &SurfaceConfiguration) {}

    fn prepare(&mut self, _margin: f32, _device: &Device, _queue: &Queue) -> NeedleErr<()> {
        Ok(())
    }

    fn render(&mut self, render_pass: &mut RenderPass) -> NeedleErr<()> {
        /* Vertex buffers without index buffer requires manual draw call. */
        render_pass.set_pipeline(&self.pipeline);
        self.buffer.submit(render_pass);

        Ok(())
    }
}
