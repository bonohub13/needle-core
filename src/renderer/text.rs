// Copyright 2025 Kensuke Saito
// SPDX-License-Identifier: GPL-2.0-or-later

use crate::{FontTypes, Fonts, NeedleErr, NeedleError, State, Text};
use glyphon::{Buffer, FontSystem, SwashCache, TextAtlas, Viewport};
use wgpu::{Device, Queue, RenderPass, SurfaceConfiguration};
use winit::dpi::PhysicalSize;

pub struct TextRenderer {
    fonts: Fonts,
    system: FontSystem,
    swash_cache: SwashCache,
    viewport: Viewport,
    atlas: TextAtlas,
    renderer: glyphon::TextRenderer,
    buffer: Buffer,
    config: Text,
    size: PhysicalSize<u32>,
}

impl TextRenderer {
    pub fn new(
        state: &State,
        config: &Text,
        font: Option<String>,
        size: &PhysicalSize<u32>,
        scale_factor: f64,
        format: wgpu::TextureFormat,
        depth_stencil: Option<wgpu::DepthStencilState>,
    ) -> NeedleErr<Self> {
        let mut fonts = Fonts::new();
        let mut system = match font {
            Some(font_name) => {
                let font = {
                    fonts.query_fonts(Some(FontTypes::Monospace))?;

                    fonts.read(&font_name)?
                };

                let mut system = FontSystem::new_with_fonts([font]);

                system.db_mut().set_monospace_family(font_name);

                system
            }
            None => FontSystem::new(),
        };
        let swash_cache = SwashCache::new();
        let cache = glyphon::Cache::new(state.device());
        let viewport = Viewport::new(state.device(), &cache);
        let mut atlas = TextAtlas::new(state.device(), state.queue(), &cache, format);
        let renderer = glyphon::TextRenderer::new(
            &mut atlas,
            state.device(),
            wgpu::MultisampleState::default(),
            depth_stencil,
        );
        let mut buffer = Buffer::new(&mut system, glyphon::Metrics::new(80.0, 60.0));
        let physical_width = (size.width as f64 * scale_factor) as f32;
        let physical_height = (size.height as f64 * scale_factor) as f32;

        buffer.set_size(&mut system, Some(physical_width), Some(physical_height));
        buffer.shape_until_scroll(&mut system, false);

        Ok(Self {
            fonts,
            system,
            swash_cache,
            viewport,
            atlas,
            renderer,
            buffer,
            config: *config,
            size: *size,
        })
    }

    #[inline]
    pub const fn scale(&self) -> f32 {
        self.config.scale
    }

    #[inline]
    pub const fn fonts_mut(&mut self) -> &mut Fonts {
        &mut self.fonts
    }

    #[inline]
    pub const fn set_config(&mut self, config: &Text) {
        self.config = *config
    }

    pub fn text_size(&self) -> [f32; 2] {
        let (width, total_lines) = self
            .buffer
            .layout_runs()
            .fold((0.0, 0usize), |(width, total_lines), run| {
                (run.line_w.max(width), total_lines + 1)
            });

        [
            width * self.scale(),
            total_lines as f32 * self.buffer.metrics().line_height * self.scale(),
        ]
    }

    pub fn set_text(&mut self, text: &str) {
        self.buffer.set_text(
            &mut self.system,
            text,
            &glyphon::Attrs::new().family(glyphon::Family::Monospace),
            glyphon::Shaping::Advanced,
        )
    }

    pub fn set_font(&mut self, font: &str) -> NeedleErr<()> {
        if self.fonts.available_fonts().is_empty() {
            self.fonts.query_fonts(Some(FontTypes::Monospace))?;
        }

        let font_src = self.fonts.read(font)?;
        let mut system = FontSystem::new_with_fonts([font_src]);

        system.db_mut().set_monospace_family(font);

        self.system = system;

        Ok(())
    }

    pub fn trim(&mut self) {
        self.atlas.trim()
    }
}

impl super::Renderer for TextRenderer {
    fn resize(&mut self, size: &PhysicalSize<u32>) {
        self.size = *size
    }

    fn update(&mut self, queue: &Queue, config: &SurfaceConfiguration) {
        self.viewport.update(
            queue,
            glyphon::Resolution {
                width: config.width,
                height: config.height,
            },
        )
    }

    fn prepare(&mut self, margin: f32, device: &Device, queue: &Queue) -> NeedleErr<()> {
        let (left, top) = self.config.position(&self.size, &self.text_size(), margin);
        let result = self.renderer.prepare(
            device,
            queue,
            &mut self.system,
            &mut self.atlas,
            &self.viewport,
            [glyphon::TextArea {
                buffer: &self.buffer,
                left,
                top,
                scale: self.config.scale,
                bounds: glyphon::TextBounds {
                    left: 0,
                    top: 0,
                    right: self.size.width as i32,
                    bottom: self.size.height as i32,
                },
                default_color: glyphon::Color::rgba(
                    self.config.color[0],
                    self.config.color[1],
                    self.config.color[2],
                    self.config.color[3],
                ),
                custom_glyphs: &[],
            }],
            &mut self.swash_cache,
        );

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(NeedleError::RendererUpdateFailure(e.into())),
        }
    }

    fn render(&mut self, render_pass: &mut RenderPass) -> NeedleErr<()> {
        match self
            .renderer
            .render(&self.atlas, &self.viewport, render_pass)
        {
            Ok(_) => Ok(()),
            Err(err) => match err {
                glyphon::RenderError::RemovedFromAtlas => Err(NeedleError::RemovedFromAtlas),
                glyphon::RenderError::ScreenResolutionChanged => {
                    Err(NeedleError::ScreenResolutionChanged)
                }
            },
        }
    }
}

impl Drop for TextRenderer {
    fn drop(&mut self) {}
}
