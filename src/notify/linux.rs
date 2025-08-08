// Copyright 2025 Kensuke Saito
// SPDX-License-Identifier: GPL-2.0-only

use crate::{DialogBackend, NotifyType};
use crate::{NeedleErr, NeedleError};
use dialog::{
    backends::{self, Dialog, KDialog, Stdio, Zenity},
    DialogBox, Message,
};
use std::fmt::{self, Display, Formatter};

pub struct Notify<Backend>
where
    Backend: DialogBox,
{
    backend: Box<Backend>,
    title: Box<str>,
}

impl<Backend> Notify<Backend>
where
    Backend: DialogBox,
    Box<Backend>: From<Box<Dialog>>,
    Box<Backend>: From<Box<KDialog>>,
    Box<Backend>: From<Box<Stdio>>,
    Box<Backend>: From<Box<Zenity>>,
{
    pub fn new(title: DialogBackend) -> Self {
        let (backend, title) = title.into();

        Self { backend, title }
    }
}

impl<Backend> NotifyType for Notify<Backend>
where
    Backend: DialogBox + backends::Backend,
{
    fn info(&self, msg: &str) -> NeedleErr<()> {
        match Message::new(msg)
            .title(self.title.as_ref())
            .show_with(&self.backend)
        {
            Ok(()) => Ok(()),
            Err(err) => Err(NeedleError::FailedToShowNotification(err)),
        }
    }

    fn warn(&self, msg: &str) -> NeedleErr<()> {
        match Message::new(msg)
            .title(self.title.as_ref())
            .show_with(&self.backend)
        {
            Ok(()) => Ok(()),
            Err(err) => Err(NeedleError::FailedToShowNotification(err)),
        }
    }

    fn error(&self, msg: &str) -> NeedleErr<()> {
        match Message::new(msg)
            .title(self.title.as_ref())
            .show_with(&self.backend)
        {
            Ok(()) => Ok(()),
            Err(err) => Err(NeedleError::FailedToShowNotification(err)),
        }
    }
}

impl Display for DialogBackend<'_> {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match self {
            &Self::Stdio(title)
            | &Self::Dialog(title, _)
            | &Self::KDialog(title)
            | &Self::Zenity(title, _) => write!(fmt, "{title}"),
        }
    }
}
