// Copyright 2025 Kensuke Saito
// SPDX-License-Identifier: GPL-2.0-or-later

use crate::{NeedleConfig, NeedleErr, NeedleLabel, ShaderDescriptor};
use serde::Deserialize;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Deserialize)]
pub struct Overlay {
    /// Path of vertex shader
    pub vertex_shader: String,
    /// Path of fragment shader
    pub fragment_shader: String,
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
}

impl Display for Overlay {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "# Vertex Shader Path : <default config shader path>/vertex_shader.spv"
        )?;
        writeln!(f, "vertex_shader = {}", self.vertex_shader,)?;
        writeln!(
            f,
            "# Fragment Shader Path : <default config shader path>/fragment_shader.spv"
        )?;
        writeln!(f, "fragment_shader = {}", self.fragment_shader,)
    }
}
