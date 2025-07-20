// Copyright 2025 Kensuke Saito
// SPDX-License-Identifier: GPL-2.0-only

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
pub use linux::Notify;
#[cfg(target_os = "windows")]
pub use windows::Notify;

use crate::NeedleErr;

pub enum DialogBackend<'title> {
    #[cfg(target_os = "linux")]
    Dialog(&'title str),
    #[cfg(target_os = "linux")]
    KDialog(&'title str),
    #[cfg(target_os = "linux")]
    Stdio(&'title str),
    #[cfg(target_os = "linux")]
    Zenity(&'title str),
    #[cfg(target_os = "windows")]
    WinApi(&'title str),
}

pub trait NotifyType {
    fn info(&self, msg: &str) -> NeedleErr<()>;
    fn warn(&self, msg: &str) -> NeedleErr<()>;
    fn error(&self, msg: &str) -> NeedleErr<()>;
}
