// Copyright 2025 Kensuke Saito
// SPDX-License-Identifier: GPL-2.0-or-later

use crate::Position;
use std::error::Error as StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NeedleError {
    // AppBase
    #[error("InitializationError | Failed to create surface. ({0})")]
    FailedToCreateSurface(wgpu::CreateSurfaceError),
    #[error("InitializationError | Failed to wgpu::Adapter supported by surface.")]
    FailedToFindValidAdapter,
    #[error("InitializationError | Failed to request device. ({0})")]
    FailedToRequestDevice(wgpu::RequestDeviceError),

    // AppConfig
    #[error("NeedleConfig | Invalid path")]
    InvalidPath,
    #[error("NeedleConfig | Config already exists")]
    ConfigExists,
    #[error("NeedleConfig | Config file doesn't exist ({0})")]
    ConfigNonExistant(Box<str>),
    #[error("NeedleConfig | Text position is invalid. Must be corners. ({0})")]
    InvalidFpsTextPosition(Position),
    #[error("NeedleConfig | Text position for FPS and time is overlapping")]
    TextPositionOverlapping,
    #[error("NeedleConfig | Failed to open config file. ({0})")]
    FailedToOpenConfig(Box<dyn StdError>),
    #[error("NeedleConfig | Failed to read config file from path. ({0})")]
    FailedToReadConfig(Box<dyn StdError>),
    #[error("NeedleConfig | Failed to parse config file. ({0})")]
    FailedToParseConfig(Box<dyn StdError>),
    #[error("NeedleConfig | Failed to write config file to path. ({0})")]
    FailedToWriteConfig(Box<dyn StdError>),
    #[error("NeedleConfig | Failed to create directory to path. ({0})")]
    FailedToCreateDirectory(Box<dyn StdError>),

    // Surface related errors
    #[error("Surface | Lost")]
    Lost,
    #[error("Surface | Outdated")]
    Outdated,
    #[error("Surface | Out of memory")]
    OutOfMemory,
    #[error("Surface | Timeout")]
    Timeout,

    // Renderer related errors
    #[error("Renderer | Removed from atlas")]
    RemovedFromAtlas,
    #[error("Renderer | Screen resolution changed")]
    ScreenResolutionChanged,
    #[error("Renderer | Buffer without bind group/bind group layout has been registered")]
    InvalidBufferRegistration,
    #[error("Renderer | Buffer without bind group/bind group layout has been registered ({0})")]
    RendererUpdateFailure(Box<dyn StdError>),
    #[error("Renderer | Failed to read specified shader file (SPIR-V: {0})")]
    FailedToReadShader(Box<dyn StdError>),

    // cURL related errors
    #[error("URL | Invalid URL format detected")]
    InvalidURLFormat,
    #[error("URL | Error detected in callback function")]
    CallbackError,
    #[error("URL | Failed to download shader")]
    ShaderDownloadFailure,
    #[error("URL | Failed to write to file")]
    WriteError,

    // Clock related errors
    #[error("Clock | Failed to start countup/countdown timer")]
    TimerStartFailure,

    // Filesystem related errors
    #[error("Filesystem | Failed to read file")]
    FailedToReadFile,
    #[error("Filesystem | Failed to read directory (Directory: {0})")]
    FailedToReadDir(Box<dyn StdError>),
    #[error("Filesystem | Failed to search for files/directories (Path: {0})")]
    FailedToSearchDir(Box<dyn StdError>),

    // Other errors
    #[error("Other | Initialization error detected")]
    InitializationError,
    #[error("Other | Invalid Regex ({0})")]
    InvalidRegex(Box<dyn StdError>),
    #[error("Other | Unknown error has been detected! Please file an issue to the repository if possible.")]
    Other,
}

unsafe impl Send for NeedleError {}
unsafe impl Sync for NeedleError {}

pub type NeedleErr<T> = Result<T, NeedleError>;
