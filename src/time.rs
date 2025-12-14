// Copyright 2025 Kensuke Saito
// SPDX-License-Identifier: GPL-2.0-or-later

use chrono::{DateTime, Local, Timelike};
use serde::Deserialize;
use std::{
    fmt::{self, Display, Formatter},
    time::{Duration, Instant},
};

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Deserialize)]
pub enum TimeFormat {
    #[default]
    HourMinSec,
    HourMinSecMSec,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
pub enum OpMode {
    Clock,
    CountDownTimer(Duration),
    CountUpTimer,
}

#[derive(Debug)]
pub struct Time {
    format: TimeFormat,
    mode: OpMode,
    start_time: Instant,
    stop_time: Option<Instant>,
    started: bool,
}

impl Time {
    const MINUTE_SECS: u64 = 60;
    const HOUR_SECS: u64 = Self::MINUTE_SECS * 60;
    const NANOSECS_IN_SECOND: u32 = 1_000_000;

    /// Create new instance of Time.
    #[inline]
    pub fn new(format: TimeFormat) -> Self {
        Self {
            format,
            mode: OpMode::Clock,
            start_time: Instant::now(),
            stop_time: None,
            started: false,
        }
    }

    /// Get current operation mode of Time.
    #[inline]
    pub fn mode(&self) -> OpMode {
        self.mode.clone()
    }

    /// Set the operation mode of Time.
    ///
    /// For operation mode(s), refer to `OpMode`
    pub fn set_mode(&mut self, mode: OpMode) {
        if self.mode != mode {
            self.mode = mode;

            match self.mode {
                OpMode::CountDownTimer(_) | OpMode::CountUpTimer => {
                    self.started = false;
                    self.start_time = Instant::now();
                }
                _ => (),
            }
        }
    }

    /// Set the display format of Time.
    ///
    /// For time format(s), refer to `TimeFormat`
    #[inline]
    pub const fn set_format(&mut self, format: TimeFormat) {
        self.format = format;
    }

    /// Starts/Stops timer (toggled state depends on current state).
    /// If timer has been started with the state below, the timer continues from previous time.
    /// 1. Started
    /// 2. Stopped (remaining duration is not 0)
    ///
    /// This only works when `OpMode` is set to either of the following.
    /// - `CountDownTimer`
    /// - `CountUpTimer`
    ///
    /// If `OpMode` is set to `Clock`, this does nothing.
    pub fn toggle_timer(&mut self) {
        match self.mode {
            OpMode::CountDownTimer(duration) => {
                self.started = !self.started;

                if self.started {
                    self.start_time = match self.stop_time {
                        Some(time) => {
                            if time - self.start_time > duration {
                                // Has been previously stopped and stopped has target duration
                                self.stop_time = None;

                                time
                            } else {
                                self.start_time
                            }
                        }
                        None => Instant::now(),
                    };
                } else {
                    self.stop_time = Some(Instant::now())
                }
            }
            OpMode::CountUpTimer => {
                self.started = !self.started;

                if self.started {
                    self.start_time = match self.stop_time {
                        Some(time) => {
                            self.stop_time = None;

                            time
                        }
                        None => Instant::now(),
                    };
                } else {
                    self.stop_time = Some(Instant::now())
                }
            }
            _ => (),
        }
    }

    /// Returns the current time/remaining duration of timer.
    /// The return value is formatted with the current format.
    /// - HourMinSec : `%H%M%S`
    /// - HourMinSecMSec : `%H%M%S%%f`
    ///
    /// For available format(s), refer to `TimeFormat`
    /// For operation mode(s), refer to `OpMode`
    pub fn current_time(&self) -> String {
        match self.mode {
            OpMode::CountDownTimer(duration) => {
                let delta = if !self.started {
                    if let Some(time) = self.stop_time {
                        time - self.start_time
                    } else {
                        Duration::new(0, 0)
                    }
                } else {
                    Instant::now() - self.start_time
                };
                let delta = if delta > duration {
                    Duration::new(0, 0)
                } else {
                    duration - delta
                };

                self.duration_to_str(&delta)
            }
            OpMode::CountUpTimer => {
                let delta = if self.started {
                    Instant::now() - self.start_time
                } else if let Some(time) = self.stop_time {
                    time - self.start_time
                } else {
                    // Count up timer hasn't been started
                    Duration::new(0, 0)
                };

                self.duration_to_str(&delta)
            }
            OpMode::Clock => self.time_to_str(&Local::now()),
        }
    }

    /// Format time into string
    fn time_to_str(&self, time: &DateTime<Local>) -> String {
        let hour = time.hour();
        let minute = time.minute();
        let second = time.second();

        match self.format {
            TimeFormat::HourMinSec => {
                format!("{hour:02}:{minute:02}:{second:02}")
            }
            TimeFormat::HourMinSecMSec => {
                let millisecond = time.nanosecond() / Self::NANOSECS_IN_SECOND;

                format!("{hour:02}:{minute:02}:{second:02}.{millisecond:03}")
            }
        }
    }

    /// Format duration into string
    fn duration_to_str(&self, delta: &Duration) -> String {
        let hour = (delta.as_secs() / Self::HOUR_SECS) as u32;
        let minute = (delta.as_secs() / Self::MINUTE_SECS) as u32;
        let second = (delta.as_secs() % Self::MINUTE_SECS) as u32;

        match self.format {
            TimeFormat::HourMinSec => {
                format!("{hour:02}:{minute:02}:{second:02}")
            }
            TimeFormat::HourMinSecMSec => {
                let millisecond = delta.as_millis() % 1000;

                format!("{hour:02}:{minute:02}:{second:02}.{millisecond:03}")
            }
        }
    }
}

impl TimeFormat {
    pub const HOUR_MIN_SEC: i8 = 0;
    pub const HOUR_MIN_SEC_MSEC: i8 = 1;
    pub const MAX: i8 = Self::HOUR_MIN_SEC_MSEC;
}

macro_rules! time_format_impl_from {
    ($type:ty) => {
        impl From<TimeFormat> for $type {
            fn from(element: TimeFormat) -> Self {
                const HOUR_MIN_SEC: $type = TimeFormat::HOUR_MIN_SEC as $type;
                const HOUR_MIN_SEC_MSEC: $type = TimeFormat::HOUR_MIN_SEC_MSEC as $type;

                match element {
                    TimeFormat::HourMinSec => HOUR_MIN_SEC,
                    TimeFormat::HourMinSecMSec => HOUR_MIN_SEC_MSEC,
                }
            }
        }

        impl From<$type> for TimeFormat {
            fn from(val: $type) -> Self {
                const HOUR_MIN_SEC: $type = TimeFormat::HOUR_MIN_SEC as $type;
                const HOUR_MIN_SEC_MSEC: $type = TimeFormat::HOUR_MIN_SEC_MSEC as $type;

                match val {
                    HOUR_MIN_SEC_MSEC => TimeFormat::HourMinSecMSec,
                    HOUR_MIN_SEC | _ => TimeFormat::HourMinSec,
                }
            }
        }
    };
}

time_format_impl_from! { i8 }
time_format_impl_from! { u8 }
time_format_impl_from! { i16 }
time_format_impl_from! { u16 }
time_format_impl_from! { i32 }
time_format_impl_from! { u32 }

impl Display for TimeFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let format = match self {
            TimeFormat::HourMinSec => "HourMinSec",
            TimeFormat::HourMinSecMSec => "HourMinSecMSec",
        };

        write!(f, "{format}")
    }
}

impl OpMode {
    pub const CLOCK: i8 = 0;
    pub const COUNT_UP_TIMER: i8 = 1;
    pub const COUNT_DOWN_TIMER: i8 = 2;
    pub const MAX: i8 = Self::COUNT_DOWN_TIMER;
}

macro_rules! op_mode_impl_from {
    ($type:ty) => {
        impl From<OpMode> for $type {
            fn from(element: OpMode) -> Self {
                const CLOCK: $type = OpMode::CLOCK as $type;
                const COUNT_UP_TIMER: $type = OpMode::COUNT_UP_TIMER as $type;
                const COUNT_DOWN_TIMER: $type = OpMode::COUNT_DOWN_TIMER as $type;

                match element {
                    OpMode::Clock => CLOCK,
                    OpMode::CountUpTimer => COUNT_UP_TIMER,
                    OpMode::CountDownTimer(_) => COUNT_DOWN_TIMER,
                }
            }
        }

        impl From<$type> for OpMode {
            fn from(val: $type) -> Self {
                const CLOCK: $type = OpMode::CLOCK as $type;
                const COUNT_UP_TIMER: $type = OpMode::COUNT_UP_TIMER as $type;
                const COUNT_DOWN_TIMER: $type = OpMode::COUNT_DOWN_TIMER as $type;

                match val {
                    COUNT_UP_TIMER => OpMode::CountUpTimer,
                    COUNT_DOWN_TIMER => OpMode::CountDownTimer(Duration::new(0, 0)),
                    CLOCK | _ => OpMode::Clock,
                }
            }
        }
    };
}

op_mode_impl_from! { i8 }
op_mode_impl_from! { u8 }
op_mode_impl_from! { i16 }
op_mode_impl_from! { u16 }
op_mode_impl_from! { i32 }
op_mode_impl_from! { u32 }

impl Display for OpMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let format = match self {
            OpMode::Clock => "Clock",
            OpMode::CountDownTimer(_) => "CountDownTimer",
            OpMode::CountUpTimer => "CountUpTimer",
        };

        write!(f, "{format}")
    }
}
