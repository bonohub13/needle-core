// Copyright 2025 Kensuke Saito
// SPDX-License-Identifier: GPL-2.0-or-later

use serde::Deserialize;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Window {
    pub fullscreen: bool,
}

impl Display for Window {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "# Fullscreen")?;
        writeln!(f, "#  true    : Enable fullscreen")?;
        writeln!(f, "#  false   : Disable fullscreen (Windowed)")?;
        writeln!(
            f,
            "fullscreen = {}",
            if self.fullscreen { "true" } else { "false" }
        )
    }
}
