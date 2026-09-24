// Copyright 2025 Kensuke Saito
// SPDX-License-Identifier: GPL-2.0-or-later

use crate::{
    NeedleErr, OverlayInfo, Position, Renderer, ShaderRenderer, ShaderRendererDescriptor, State,
    Text, TextRenderer, TextRendererDescriptor,
};
use winit::dpi::PhysicalSize;

pub struct OverlayRenderer {
    text_renderer: TextRenderer,
    shader_renderer: ShaderRenderer,
    info: OverlayInfo,
}

impl OverlayRenderer {
    #[inline]
    pub fn new(
        state: &State,
        shader_desc: &ShaderRendererDescriptor,
        text_desc: &TextRendererDescriptor,
        overlay_info: &OverlayInfo,
    ) -> NeedleErr<Self> {
        let shader_renderer = ShaderRenderer::new(state, shader_desc)?;
        let text_renderer = TextRenderer::new(state, text_desc)?;

        Ok(Self {
            shader_renderer,
            text_renderer,
            info: *overlay_info,
        })
    }

    #[inline]
    pub const fn info(&self) -> &OverlayInfo {
        &self.info
    }

    #[inline]
    pub const fn set_position(&mut self, top_left: [f32; 2]) {
        self.info.set_top_left(top_left)
    }

    #[inline]
    pub const fn set_size(&mut self, size: [f32; 2]) {
        self.info.set_size(size)
    }

    #[inline]
    pub const fn set_color(&mut self, color: [f32; 4]) {
        self.info.set_color(color)
    }

    #[inline]
    pub fn set_text(&mut self, text: &str) {
        self.text_renderer.set_text(text)
    }

    #[inline]
    pub fn set_font(&mut self, font: &str) -> NeedleErr<()> {
        self.text_renderer.set_font(font)
    }

    #[inline]
    pub fn set_font_color(&mut self, config: &Text) {
        self.text_renderer.set_config(config)
    }

    #[inline]
    pub const fn set_font_position(&mut self, position: [f32; 2]) {
        let overlay_top_left = self.info.top_left();
        let font_bottom_right = {
            let size = self.text_renderer.size;
            let position = if let Position::Coordinate { x, y } = self.text_renderer.config.position
            {
                glm::vec2(x, y)
            } else {
                *overlay_top_left
            };

            glm::vec2(
                size.width as f32 + position.x,
                size.height as f32 + position.y,
            )
        };
        let overlay_bottom_right = {
            let size = self.info.size();

            glm::vec2(overlay_top_left.x + size.x, overlay_top_left.y + size.y)
        };

        if (font_bottom_right.x <= overlay_bottom_right.x)
            && (font_bottom_right.y <= overlay_bottom_right.y)
        {
            let mut config = self.text_renderer.config;

            config.position = Position::Coordinate {
                x: position[0],
                y: position[1],
            };
            self.text_renderer.set_config(&config);
        }
    }

    pub fn write_buffer<T>(&mut self, data: &T, queue: &wgpu::Queue) -> NeedleErr<()>
    where
        T: Sized + Clone,
    {
        self.shader_renderer.write_buffer(data, queue)
    }
}

impl Renderer for OverlayRenderer {
    fn resize(&mut self, _size: &PhysicalSize<u32>) {}

    fn update(&mut self, state: &State) {
        self.text_renderer.update(state)
    }

    fn prepare(&mut self, margin: f32, state: &State) -> NeedleErr<()> {
        self.text_renderer.prepare(margin, state)
    }

    fn render(&mut self, render_pass: &mut wgpu::RenderPass) -> NeedleErr<()> {
        self.shader_renderer.render(render_pass)?;
        self.text_renderer.render(render_pass)
    }
}
