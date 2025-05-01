// Copyright 2025 Kensuke Saito
// SPDX-License-Identifier: GPL-2.0-only

mod fps;
mod position;
mod text;
mod time;

pub use fps::*;
pub use position::*;
pub use text::*;
pub use time::*;

use crate::{
    error::{NeedleErr, NeedleError},
    TimeFormat,
};
use directories::ProjectDirs;
use serde::Deserialize;
use std::{
    ffi::OsStr,
    fmt::{self, Display, Formatter},
    fs::{self, OpenOptions},
    io::{BufReader, BufWriter, Read, Write},
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Deserialize)]
pub struct NeedleConfig {
    pub background_color: [f32; 4],
    pub time: TimeConfig,
    pub fps: FpsConfig,
}

impl<'a> NeedleConfig {
    #[cfg(windows)]
    const NEWLINE: &'a str = "\r\n";
    #[cfg(not(windows))]
    const NEWLINE: &'a str = "\n";
    const CONFIG_FILE: &'a str = "config.toml";

    pub fn config(path: Option<&str>) -> NeedleErr<()> {
        let default_config_file = Self::config_file(true)?;
        let config_file = if let Some(path) = path {
            if path.is_empty() {
                &default_config_file
            } else {
                Path::new(path)
            }
        } else {
            &default_config_file
        };

        Self::write(&config_file)
    }

    pub fn from(path: Option<&str>) -> NeedleErr<Self> {
        let default_config_file = Self::config_file(false)?;
        let config_file = if let Some(path) = path {
            if path.is_empty() {
                &default_config_file
            } else {
                Path::new(path)
            }
        } else {
            &default_config_file
        };

        if !config_file.exists() {
            if config_file == &default_config_file {
                Self::config(None)?;
            } else {
                let config_file = config_file.to_string_lossy();

                return Err(NeedleError::ConfigNonExistant(config_file.into()));
            }
        }

        let read = match OpenOptions::new().read(true).open(config_file) {
            Ok(file) => Ok(file),
            Err(err) => Err(NeedleError::FailedToOpenConfig(err.into())),
        }?;
        let mut buf_reader = BufReader::new(read);
        let mut read_buffer = String::new();

        match buf_reader.read_to_string(&mut read_buffer) {
            Ok(_) => Ok(()),
            Err(err) => Err(NeedleError::FailedToReadConfig(err.into())),
        }?;

        let config: Self = match toml::from_str(&read_buffer) {
            Ok(toml) => Ok(toml),
            Err(err) => Err(NeedleError::FailedToParseConfig(err.into())),
        }?;

        if config.fps.enable && !config.fps.is_valid_position() {
            Err(NeedleError::InvalidFpsTextPosition(
                config.fps.config.position,
            ))
        } else if config.fps.enable && (config.fps.config.position == config.time.config.position) {
            Err(NeedleError::TextPositionOverlapping)
        } else {
            Ok(config)
        }
    }

    pub fn config_path(create_dir: bool, relative_path: Option<&str>) -> NeedleErr<PathBuf> {
        let mut config_path: PathBuf;
        let relative_path = match relative_path {
            Some(path) => Ok(path.split("/").collect::<Vec<_>>()),
            None => Err(NeedleError::InvalidPath),
        }?;

        match ProjectDirs::from("com", "bonohub13", "needle") {
            Some(app_dir) => {
                if (!app_dir.config_dir().exists()) && create_dir {
                    match fs::create_dir_all(app_dir.config_dir()) {
                        Ok(_) => Ok(()),
                        Err(err) => Err(NeedleError::FailedToCreateDirectory(err.into())),
                    }?;
                }

                config_path = app_dir.config_dir().to_path_buf();
            }
            None => return Err(NeedleError::InvalidPath),
        }

        for rpath in relative_path {
            match rpath {
                "." | "" | " " | "\t" => (),
                ".." => {
                    if !config_path.pop() {
                        return Err(NeedleError::InvalidPath);
                    }
                }
                _ => config_path.push(rpath),
            }
        }

        Ok(config_path)
    }

    fn config_file(create_dir: bool) -> NeedleErr<PathBuf> {
        Self::config_path(create_dir, Some(Self::CONFIG_FILE))
    }

    fn write(file: &Path) -> NeedleErr<()> {
        let default_config_path = Self::config_file(false)?;
        if file.exists() && file == default_config_path.as_path() {
            return Err(NeedleError::ConfigExists);
        }

        let config = Self::default();

        if file.as_os_str() == OsStr::new("stdout") {
            println!("{}", config);

            Ok(())
        } else {
            let file = match OpenOptions::new().write(true).create(true).open(file) {
                Ok(file) => Ok(file),
                Err(err) => Err(NeedleError::FailedToWriteConfig(err.into())),
            }?;
            let mut buf_writer = BufWriter::new(file);

            match writeln!(buf_writer, "{}", config) {
                Ok(_) => Ok(()),
                Err(err) => Err(NeedleError::FailedToWriteConfig(err.into())),
            }
        }
    }
}

impl Default for NeedleConfig {
    fn default() -> Self {
        Self {
            background_color: [0.0, 0.0, 0.0, 1.0],
            time: TimeConfig {
                format: TimeFormat::HourMinSec,
                font: None,
                config: Text {
                    scale: 1.0,
                    color: [255, 255, 255, 255],
                    position: Position::Center,
                },
            },
            fps: FpsConfig {
                enable: false,
                frame_limit: 30,
                config: Text {
                    scale: 0.25,
                    color: [255, 0, 0, 255],
                    position: Position::TopRight,
                },
            },
        }
    }
}

impl Display for NeedleConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "# Background color : [r, g, b, alpha]")?;
        writeln!(f, "#  Range : (0.0 - 1.0)")?;
        writeln!(
            f,
            "background_color = [{}, {}, {}, {}]",
            self.background_color[0],
            self.background_color[1],
            self.background_color[2],
            self.background_color[3]
        )?;
        writeln!(f, "{}[time]", Self::NEWLINE)?;
        writeln!(f, "{}", self.time)?;
        writeln!(f, "{}[fps]", Self::NEWLINE)?;
        write!(f, "{}", self.fps)
    }
}
