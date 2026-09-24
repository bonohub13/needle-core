// Copyright 2025 Kensuke Saito
// SPDX-License-Identifier: GPL-2.0-or-later

use crate::{BindGroupLayout, Buffer, NeedleErr, NeedleError, NeedleLabel, State, Ubo};
use std::{
    fs::OpenOptions,
    io::Read,
    path::{Path, PathBuf},
};
use wgpu::{Queue, RenderPass, RenderPipeline};
use winit::dpi::PhysicalSize;

#[derive(Debug, Clone)]
pub struct ShaderDescriptor<'a> {
    /// Path to vertex shader
    pub vertex: PathBuf,
    /// Label for vertex shader
    pub vertex_label: NeedleLabel<'a>,
    /// Path to fragment shader
    pub fragment: PathBuf,
    /// Label for fragment shader
    pub fragment_label: NeedleLabel<'a>,
}

#[derive(Debug)]
pub struct ShaderRendererDescriptor<'desc> {
    /// Shader Descriptor
    pub shader_desc: ShaderDescriptor<'desc>,
    /// Buffer
    pub buffer: Buffer,
    /// UBO
    pub ubo: Option<Ubo>,
    /// Buffer layouts of Vertex buffer
    pub vertex_buffer_layout: wgpu::VertexBufferLayout<'desc>,
    /// Bind group layouts
    pub bind_group_layouts: Vec<BindGroupLayout>,
    /// Depth Stencil
    pub depth_stencil: Option<wgpu::DepthStencilState>,
    /// Label used for vertex buffer and index buffer
    pub label: Option<&'desc str>,
}

#[derive(Debug, Clone)]
pub struct Shader {
    vertex: wgpu::ShaderModule,
    fragment: wgpu::ShaderModule,
}

impl Shader {
    pub fn new(state: &State, desc: &ShaderDescriptor) -> NeedleErr<Self> {
        let vert_shader_code = Self::read_shader(&desc.vertex)?;
        let frag_shader_code = Self::read_shader(&desc.fragment)?;
        let vertex = unsafe {
            state.device().create_shader_module_passthrough(
                wgpu::ShaderModuleDescriptorPassthrough::SpirV(wgpu::ShaderModuleDescriptorSpirV {
                    label: Some(&desc.vertex_label.to_string()),
                    source: wgpu::util::make_spirv_raw(&vert_shader_code),
                }),
            )
        };
        let fragment = unsafe {
            state.device().create_shader_module_passthrough(
                wgpu::ShaderModuleDescriptorPassthrough::SpirV(wgpu::ShaderModuleDescriptorSpirV {
                    label: Some(&desc.fragment_label.to_string()),
                    source: wgpu::util::make_spirv_raw(&frag_shader_code),
                }),
            )
        };

        Ok(Self { vertex, fragment })
    }

    fn read_shader(path: &Path) -> NeedleErr<Box<[u8]>> {
        const BYTE_ALIGNMENT: usize = 4; // Check and enforce 4byte alignment

        let mut reader = match OpenOptions::new().read(true).open(path) {
            Ok(file) => Ok(file),
            Err(err) => Err(NeedleError::FailedToReadShader(err.into())),
        }?;
        let mut buffer = vec![];

        match reader.read_to_end(&mut buffer) {
            Ok(_) => Ok(()),
            Err(err) => Err(NeedleError::FailedToReadShader(err.into())),
        }?;
        if buffer.len().is_multiple_of(BYTE_ALIGNMENT) {
            // Append 0 to buffer to force 4byte alignment
            buffer.extend(std::iter::repeat_n(0, buffer.len() % BYTE_ALIGNMENT));
        }

        let buffer = Box::from_iter(buffer);

        Ok(buffer)
    }
}

#[derive(Debug)]
pub struct ShaderRenderer {
    buffer: Buffer,
    ubo: Option<Ubo>,
    pipeline: RenderPipeline,
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
        let bind_group_layouts = desc
            .bind_group_layouts
            .iter()
            .map(|layout| layout.layout())
            .collect::<Vec<_>>();
        let render_pipeline_layout =
            state
                .device()
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some(&NeedleLabel::PipelineLayout(&label).to_string()),
                    bind_group_layouts: &bind_group_layouts,
                    push_constant_ranges: &[],
                });
        let shader = Shader::new(state, &desc.shader_desc)?;
        let render_pipeline =
            state
                .device()
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some(&NeedleLabel::Pipeline(&label).to_string()),
                    layout: Some(&render_pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &shader.vertex,
                        entry_point: Some("main"),
                        buffers: std::slice::from_ref(&desc.vertex_buffer_layout),
                        compilation_options: wgpu::PipelineCompilationOptions::default(),
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &shader.fragment,
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
            ubo: desc.ubo.clone(),
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

    /// Update UBO.
    /// Does nothing if renderer doesn't have an UBO.
    pub fn write_buffer<T>(&mut self, data: &T, queue: &Queue) -> NeedleErr<()>
    where
        T: Sized + Clone,
    {
        if let Some(ubo) = &mut self.ubo {
            ubo.update(data, queue);
        }

        Ok(())
    }
}

impl super::Renderer for ShaderRenderer {
    fn resize(&mut self, _size: &PhysicalSize<u32>) {}

    fn update(&mut self, _state: &State) {}

    fn prepare(&mut self, _margin: f32, _state: &State) -> NeedleErr<()> {
        Ok(())
    }

    fn render(&mut self, render_pass: &mut RenderPass) -> NeedleErr<()> {
        /* Vertex buffers without index buffer requires manual draw call. */
        render_pass.set_pipeline(&self.pipeline);

        self.buffer.submit(render_pass);
        if let Some(ubo) = &self.ubo {
            ubo.submit(render_pass);
        }
        self.buffer.draw(render_pass);

        Ok(())
    }
}
