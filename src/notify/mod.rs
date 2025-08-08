// Copyright 2025 Kensuke Saito
// SPDX-License-Identifier: GPL-2.0-only

#[cfg(target_os = "linux")]
use dialog::{
    backends::{Dialog, KDialog, Stdio, Zenity},
    DialogBox,
};

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
    Dialog(&'title str, [u32; 2]),
    #[cfg(target_os = "linux")]
    KDialog(&'title str),
    #[cfg(target_os = "linux")]
    Stdio(&'title str),
    #[cfg(target_os = "linux")]
    Zenity(&'title str, [u32; 2]),
    #[cfg(target_os = "windows")]
    WinApi(&'title str),
}

pub trait NotifyType {
    fn info(&self, msg: &str) -> NeedleErr<()>;
    fn warn(&self, msg: &str) -> NeedleErr<()>;
    fn error(&self, msg: &str) -> NeedleErr<()>;
}

#[cfg(target_os = "linux")]
impl<'title, Backend> From<DialogBackend<'title>> for (Box<Backend>, Box<str>)
where
    Backend: DialogBox,
    Box<Backend>: From<Box<Dialog>>,
    Box<Backend>: From<Box<KDialog>>,
    Box<Backend>: From<Box<Stdio>>,
    Box<Backend>: From<Box<Zenity>>,
{
    fn from(backend: DialogBackend<'title>) -> Self {
        match backend {
            DialogBackend::Dialog(title, size) => {
                let mut backend = Dialog::new();

                backend.set_width(size[0]);
                backend.set_height(size[1]);

                (Box::new(backend).into(), title.into())
            }
            DialogBackend::KDialog(title) => (Box::new(KDialog::new()).into(), title.into()),
            DialogBackend::Stdio(title) => (Box::new(Stdio::new()).into(), title.into()),
            DialogBackend::Zenity(title, size) => {
                let mut backend = Zenity::new();

                backend.set_width(size[0]);
                backend.set_height(size[1]);

                (Box::new(backend).into(), title.into())
            }
        }
    }
}

#[cfg(target_os = "windows")]
impl<'title, Backend> From<DialogBackend<'title>> for Box<str> {
    fn from(backend: DialogBackend<'title>) -> Self {
        match backend {
            DialogBackend::WinApi(title) => title.into(),
        }
    }
}
