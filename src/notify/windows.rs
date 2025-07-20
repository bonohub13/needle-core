// Copyright 2025 Kensuke Saito
// SPDX-License-Identifier: GPL-2.0-only

use super::{DialogBackend, NotifyType};
use crate::{NeedleErr, NeedleError};
use win_msgbox::{MessageBox, Okay};

pub struct Notify {
    title: Box<str>,
}

impl Notify {
    pub fn new(title: DialogBackend) -> Self {
        let title: Box<str> = match title {
            DialogBackend::WinApi(title) => title.into(),
        };

        Self { title }
    }

    fn show<Opt>(msgbox: MessageBox<Opt>) -> NeedleErr<()>
    where
        Opt: win_msgbox::Options,
    {
        match msgbox.show() {
            Ok(_) => Ok(()),
            Err(err) => Err(NeedleError::FailedToShowWinNotification(err)),
        }
    }
}

impl NotifyType for Notify {
    fn info(&self, msg: &str) -> NeedleErr<()> {
        let msgbox = win_msgbox::information::<Okay>(msg).title(&self.title);

        Self::show(msgbox)
    }

    fn error(&self, msg: &str) -> NeedleErr<()> {
        let msgbox = win_msgbox::error::<Okay>(msg).title(&self.title);

        Self::show(msgbox)
    }

    fn warn(&self, msg: &str) -> NeedleErr<()> {
        let msgbox = win_msgbox::warning::<Okay>(msg).title(&self.title);

        Self::show(msgbox)
    }
}
