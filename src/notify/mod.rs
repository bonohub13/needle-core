// Copyright 2025 Kensuke Saito
// SPDX-License-Identifier: GPL-2.0-only

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
pub use linux::{DialogBackend, Notify};
#[cfg(target_os = "windows")]
pub use windows::Notify;

use crate::NeedleErr;

pub trait NotifyType {
    fn info(&self, msg: &str) -> NeedleErr<()>;
    fn warn(&self, msg: &str) -> NeedleErr<()>;
    fn error(&self, msg: &str) -> NeedleErr<()>;
}
