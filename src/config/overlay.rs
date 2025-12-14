// Copyright 2025 Kensuke Saito
// SPDX-License-Identifier: GPL-2.0-or-later

use crate::{NeedleConfig, NeedleErr, NeedleLabel, OverlayInfo, ShaderDescriptor};
use serde::Deserialize;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Default, Clone, PartialEq, Deserialize)]
pub struct Overlay {
    pub name: String,
    /// Path of vertex shader
    pub vertex_shader: String,
    /// Path of fragment shader
    pub fragment_shader: String,
    /// Position: [x, y]
    pub position: [f32; 2],
    /// Size: [x, y]
    pub size: [f32; 2],
    /// Color: [r, g, b, a]
    pub color: [f32; 4],
}

impl Overlay {
    const SHADER_BASE: &'static str = "shaders/spv";

    pub fn shader_descriptor<'label>(
        &self,
        vertex_label: NeedleLabel<'label>,
        fragment_label: NeedleLabel<'label>,
    ) -> NeedleErr<Option<ShaderDescriptor<'label>>> {
        if self.vertex_shader.is_empty() || self.fragment_shader.is_empty() {
            Ok(None)
        } else {
            let vertex = NeedleConfig::config_path(
                false,
                Some(&format!("{}/{}", Self::SHADER_BASE, self.vertex_shader)),
            )?;
            let fragment = NeedleConfig::config_path(
                false,
                Some(&format!("{}/{}", Self::SHADER_BASE, self.fragment_shader)),
            )?;

            Ok(Some(ShaderDescriptor {
                vertex,
                vertex_label,
                fragment,
                fragment_label,
            }))
        }
    }

    #[inline]
    pub const fn info(&self) -> OverlayInfo {
        OverlayInfo::new(self.position, self.size, self.color)
    }
}

impl Display for Overlay {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "name = \"{}\", ", self.name,)?;
        write!(f, "vertex_shader = \"{}\", ", self.vertex_shader,)?;
        write!(f, "fragment_shader = \"{}\", ", self.fragment_shader,)?;
        write!(
            f,
            "position = [{}, {}], ",
            self.position[0], self.position[1]
        )?;
        write!(f, "size = [{}, {}], ", self.size[0], self.size[1])?;
        write!(
            f,
            "color = [{}, {}, {}, {}]",
            self.color[0], self.color[1], self.color[2], self.color[3]
        )
    }
}
