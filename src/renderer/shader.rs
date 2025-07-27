// Copyright 2025 Kensuke Saito
// SPDX-License-Identifier: GPL-2.0-or-later

use crate::{NeedleErr, NeedleError, NeedleLabel, State};
use std::{
    fs::OpenOptions,
    io::Read,
    path::{Path, PathBuf},
};
use wgpu::{Buffer, Device, Queue, RenderPass, RenderPipeline, ShaderModule, SurfaceConfiguration};
use winit::dpi::PhysicalSize;

pub struct ShaderRenderer {
    _vert_shader: ShaderModule,
    _frag_shader: ShaderModule,
    _vert_shader_code: Box<[u8]>,
    _frag_shader_code: Box<[u8]>,
    vertex_buffers: Vec<Buffer>,
    indices: Option<(i32, Box<[u16]>)>,
    index_buffers: Option<Buffer>,
    pipeline: RenderPipeline,
}

#[derive(Debug)]
pub struct ShaderRendererDescriptor<'desc> {
    pub vert_shader_path: PathBuf,
    pub frag_shader_path: PathBuf,
    pub vertex_buffers: &'desc [wgpu::Buffer],
    pub vertex_buffer_layouts: &'desc [wgpu::VertexBufferLayout<'desc>],
    pub indices: Option<(i32, Box<[u16]>)>,
    pub index_buffers: Option<wgpu::Buffer>,
    pub depth_stencil: Option<wgpu::DepthStencilState>,
    pub label: Option<&'desc str>,
}

impl ShaderRenderer {
    pub fn new(state: &State, desc: &ShaderRendererDescriptor) -> NeedleErr<Self> {
        // Each buffer must have their bind group layout and bind group
        if desc.vertex_buffers.len() != desc.vertex_buffer_layouts.len() {
            return Err(NeedleError::InvalidBufferRegistration);
        }
        if desc.indices.is_some() != desc.index_buffers.is_some() {
            return Err(NeedleError::InvalidBufferRegistration);
        }

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
                        buffers: desc.vertex_buffer_layouts,
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
            _vert_shader_code: vert_shader_code,
            _frag_shader_code: frag_shader_code,
            _vert_shader: vert_shader,
            _frag_shader: frag_shader,
            vertex_buffers: desc.vertex_buffers.to_vec(),
            indices: desc.indices.clone(),
            index_buffers: desc.index_buffers.clone(),
            pipeline: render_pipeline,
        })
    }

    #[inline]
    pub const fn pipeline(&self) -> &RenderPipeline {
        &self.pipeline
    }

    #[inline]
    pub fn vertex_buffer(&self, index: usize) -> &Buffer {
        &self.vertex_buffers[index]
    }

    pub fn set_vertex_buffer(&mut self, buffer: &[Buffer]) -> NeedleErr<()> {
        if buffer.len() != self.vertex_buffers.len() {
            Err(NeedleError::InvalidBufferRegistration)
        } else {
            self.vertex_buffers.iter_mut().for_each(|buf| {
                buf.destroy();
            });
            self.vertex_buffers = buffer.to_vec();

            Ok(())
        }
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
        for (i, vertex_buffer) in self.vertex_buffers.iter().enumerate() {
            render_pass.set_vertex_buffer(i as u32, vertex_buffer.slice(..));
        }
        if let (Some(index_buffer), Some((base_vertex, indices))) =
            (self.index_buffers.as_ref(), self.indices.as_ref())
        {
            render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..indices.len() as u32, *base_vertex, 0..1);
        }

        Ok(())
    }
}
