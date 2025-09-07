// Copyright 2025 Kensuke Saito
// SPDX-License-Identifier: GPL-2.0-or-later

mod base;
mod bind_group;
mod buffers;
mod config;
mod error;
mod notify;
mod renderer;
mod texture;
mod time;
mod utils;

pub use base::*;
pub use bind_group::*;
pub use buffers::*;
pub use config::*;
pub use error::*;
pub use notify::NotifyType;
pub use renderer::*;
pub use texture::*;
pub use time::*;
pub use utils::*;

#[cfg(target_os = "windows")]
pub use notify::Notify;
#[cfg(target_os = "linux")]
pub use notify::{DialogBackend, Notify};

use std::fmt::{Display, Formatter, Result};

/// Returns version information.
/// - Package name
/// - Version
pub fn version_info() -> String {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");

    format!("{name} {version}")
}

/// Labels for wgpu/imgui objects in needle
#[derive(Debug, Clone, Copy)]
pub enum NeedleLabel<'a> {
    ImguiWindow(&'a str),
    Device(&'a str),
    PipelineLayout(&'a str),
    Pipeline(&'a str),
    CommandEncoder(&'a str),
    RenderPass(&'a str),
    Renderer(&'a str),
    Shader(&'a str),
    Texture(&'a str),
    Buffer(&'a str),
    UniformBuffer(&'a str),
    BindGroupLayout(&'a str),
    BindGroup(&'a str),
}

impl<'a> Display for NeedleLabel<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let label = match self {
            Self::ImguiWindow(label) => {
                if label.is_empty() {
                    "Imgui Window".to_string()
                } else {
                    format!("{label} Imgui Window")
                }
            }
            Self::Device(label) => {
                if label.is_empty() {
                    "Device".to_string()
                } else {
                    format!("{label} Device")
                }
            }
            Self::PipelineLayout(label) => {
                if label.is_empty() {
                    "Pipeline Layout".to_string()
                } else {
                    format!("{label} Pipeline Layout")
                }
            }
            Self::Pipeline(label) => {
                if label.is_empty() {
                    "Render Pipeline".to_string()
                } else {
                    format!("{label} Pipeline")
                }
            }
            Self::CommandEncoder(label) => {
                if label.is_empty() {
                    "Command Encoder".to_string()
                } else {
                    format!("{label} Command Encoder")
                }
            }
            Self::RenderPass(label) => {
                if label.is_empty() {
                    "Render Pass".to_string()
                } else {
                    format!("{label} Render Pass")
                }
            }
            Self::Renderer(label) => {
                if label.is_empty() {
                    "Renderer".to_string()
                } else {
                    format!("{label} Renderer")
                }
            }
            Self::Shader(label) => {
                if label.is_empty() {
                    "Shader".to_string()
                } else {
                    format!("{label} Shader")
                }
            }
            Self::Texture(label) => {
                if label.is_empty() {
                    "Texture".to_string()
                } else {
                    format!("{label} Texture")
                }
            }
            Self::Buffer(label) => {
                if label.is_empty() {
                    "Buffer".to_string()
                } else {
                    format!("{label} Buffer")
                }
            }
            Self::UniformBuffer(label) => {
                if label.is_empty() {
                    "Uniform Buffer".to_string()
                } else {
                    format!("{label} Uniform Buffer")
                }
            }
            Self::BindGroupLayout(label) => {
                if label.is_empty() {
                    "Bind Group Layout".to_string()
                } else {
                    format!("{label} Bind Group Layout")
                }
            }
            Self::BindGroup(label) => {
                if label.is_empty() {
                    "Bind Group".to_string()
                } else {
                    format!("{label} Bind Group")
                }
            }
        };

        write!(f, "{label}")
    }
}
