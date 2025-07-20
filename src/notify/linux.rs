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
    Box<Backend>: From<Dialog>,
    Box<Backend>: From<KDialog>,
    Box<Backend>: From<Stdio>,
    Box<Backend>: From<Zenity>,
{
    const WIDTH: u32 = 100;
    const HEIGHT: u32 = 60;

    pub fn new(title: DialogBackend) -> Self {
        let (backend, title): (Box<Backend>, Box<str>) = match title {
            DialogBackend::Dialog(title) => {
                let mut backend = Dialog::new();

                backend.set_width(Self::WIDTH);
                backend.set_height(Self::HEIGHT);

                (backend.into(), title.into())
            }
            DialogBackend::KDialog(title) => (KDialog::new().into(), title.into()),
            DialogBackend::Stdio(title) => (Stdio::new().into(), title.into()),
            DialogBackend::Zenity(title) => {
                let mut backend = Zenity::new();

                backend.set_width(Self::WIDTH);
                backend.set_height(Self::HEIGHT);

                (backend.into(), title.into())
            }
        };

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
            | &Self::Dialog(title)
            | &Self::KDialog(title)
            | &Self::Zenity(title) => write!(fmt, "{title}"),
        }
    }
}
