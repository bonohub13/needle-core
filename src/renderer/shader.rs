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
        let vertex = state
            .device()
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some(desc.vertex_label.to_string().as_str()),
                source: wgpu::ShaderSource::SpirV(std::borrow::Cow::Borrowed(&vert_shader_code)),
            });
        let fragment = state
            .device()
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some(desc.vertex_label.to_string().as_str()),
                source: wgpu::ShaderSource::SpirV(std::borrow::Cow::Borrowed(&frag_shader_code)),
            });

        Ok(Self { vertex, fragment })
    }

    fn read_shader(path: &Path) -> NeedleErr<Box<[u32]>> {
        const BYTE_ALIGNMENT: usize = size_of::<u32>(); // Check and enforce 4byte alignment

        let mut reader = OpenOptions::new()
            .read(true)
            .open(path)
            .map_err(|err| NeedleError::FailedToReadShader(err.into()))?;
        let mut buffer = vec![];

        reader
            .read_to_end(&mut buffer)
            .map_err(|err| NeedleError::FailedToReadShader(err.into()))?;
        if !buffer.len().is_multiple_of(BYTE_ALIGNMENT) {
            // Append 0 to buffer to force 4byte alignment
            buffer.extend(std::iter::repeat_n(0, buffer.len() % BYTE_ALIGNMENT));
        }

        if buffer.as_ptr() as usize & BYTE_ALIGNMENT == 0 {
            let u32_slice = unsafe {
                std::slice::from_raw_parts(
                    buffer.as_ptr() as *const u32,
                    buffer.len() / size_of::<u32>(),
                )
            };

            Ok(u32_slice.into())
        } else {
            let u32_vec: Vec<u32> = buffer
                .as_chunks::<{ size_of::<u32>() }>()
                .0
                .iter()
                .map(|chunk| u32::from_le_bytes(*chunk))
                .collect();

            Ok(u32_vec.as_slice().into())
        }
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
        let label = desc.label.unwrap_or("Render");
        let bind_group_layouts = desc
            .bind_group_layouts
            .iter()
            .map(|layout| Some(layout.layout()))
            .collect::<Vec<_>>();
        let render_pipeline_layout =
            state
                .device()
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some(&NeedleLabel::PipelineLayout(label).to_string()),
                    bind_group_layouts: &bind_group_layouts,
                    ..Default::default()
                });
        let shader = Shader::new(state, &desc.shader_desc)?;
        let render_pipeline =
            state
                .device()
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some(&NeedleLabel::Pipeline(label).to_string()),
                    layout: Some(&render_pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &shader.vertex,
                        entry_point: Some("main"),
                        buffers: std::slice::from_ref(&Some(desc.vertex_buffer_layout.clone())),
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
                    multiview_mask: None,
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
