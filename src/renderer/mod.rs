// Copyright 2025 Kensuke Saito
// SPDX-License-Identifier: GPL-2.0-or-later

mod overlay;
mod shader;
mod text;

pub use overlay::*;
pub use shader::*;
pub use text::*;

use crate::{NeedleErr, State};
use wgpu::RenderPass;
use winit::dpi::PhysicalSize;

pub trait Renderer {
    fn resize(&mut self, size: &PhysicalSize<u32>);
    fn update(&mut self, state: &State);
    fn prepare(&mut self, margin: f32, state: &State) -> NeedleErr<()>;
    fn render(&mut self, render_pass: &mut RenderPass) -> NeedleErr<()>;
}
