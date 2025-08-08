// Copyright 2025 Kensuke Saito
// SPDX-License-Identifier: GPL-2.0-or-later

use crate::{NeedleConfig, NeedleErr, NeedleError, NeedleLabel, State};
use imgui::{Context, FontConfig, FontSource, MouseCursor};
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use std::{
    cell::RefCell,
    error::Error as StdError,
    fmt::{self, Display, Formatter},
    rc::Rc,
    sync::Arc,
    time::Instant,
};
use winit::window::{Window, WindowId};

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum ImguiMode {
    Background,
    ClockTimer,
    Fps,
    Invalid,
}

macro_rules! imgui_mode_from {
    ( $type:ty ) => {
        impl From<ImguiMode> for $type {
            fn from(mode: ImguiMode) -> Self {
                match mode {
                    ImguiMode::Background => 0,
                    ImguiMode::ClockTimer => 1,
                    ImguiMode::Fps => 2,
                    ImguiMode::Invalid => Self::MAX,
                }
            }
        }

        impl From<$type> for ImguiMode {
            fn from(val: $type) -> Self {
                match val {
                    0 => ImguiMode::Background,
                    1 => ImguiMode::ClockTimer,
                    2 => ImguiMode::Fps,
                    _ => ImguiMode::Invalid,
                }
            }
        }
    };
}

imgui_mode_from! { i8 }
imgui_mode_from! { u8 }
imgui_mode_from! { i16 }
imgui_mode_from! { u16 }
imgui_mode_from! { i32 }
imgui_mode_from! { u32 }

impl Display for ImguiMode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Background => write!(f, "Background"),
            Self::ClockTimer => write!(f, "Clock/Timer"),
            Self::Fps => write!(f, "FPS"),
            Self::Invalid => write!(f, "Invalid"),
        }
    }
}

pub struct ImguiState {
    context: Context,
    platform: WinitPlatform,
    renderer: imgui_wgpu::Renderer,
    last_cursor: Option<MouseCursor>,
    last_frame: Instant,
    show_imgui: bool,
    settings_mode: ImguiMode,
}

impl ImguiState {
    pub fn new(window: Arc<Window>, config: Rc<RefCell<NeedleConfig>>, state: &State) -> Self {
        let mut context = Self::create_context(window.clone(), config.clone());
        let platform = Self::create_platform(window.clone(), &mut context);
        let renderer = Self::create_renderer(&mut context, state);

        Self {
            context,
            platform,
            renderer,
            last_cursor: None,
            last_frame: Instant::now(),
            show_imgui: true,
            settings_mode: ImguiMode::Background,
        }
    }

    pub fn update(&mut self, new_frame: Instant) {
        self.context
            .io_mut()
            .update_delta_time(new_frame - self.last_frame);
        self.last_frame = new_frame;
    }

    pub fn setup<SetupFn, Err>(&mut self, window: &Window, setup: SetupFn) -> NeedleErr<()>
    where
        SetupFn: FnOnce(&mut imgui::Ui, &mut ImguiMode) -> Result<(), Err>,
        Err: StdError + Into<Box<dyn StdError>>,
    {
        match self.platform.prepare_frame(self.context.io_mut(), window) {
            Ok(_) => Ok(()),
            Err(err) => Err(NeedleError::FailedToPrepareUiFrame(err.into())),
        }?;
        let ui = self.context.new_frame();

        if self.show_imgui {
            match setup(ui, &mut self.settings_mode) {
                Ok(_) => Ok(()),
                Err(err) => Err(NeedleError::FailedToSetupUi(err.into())),
            }?;
        }

        if self.last_cursor != ui.mouse_cursor() {
            self.last_cursor = ui.mouse_cursor();
            self.platform.prepare_render(ui, window);
        }

        Ok(())
    }

    pub fn render(&mut self, state: &State, view: &wgpu::TextureView) -> NeedleErr<()> {
        let mut encoder = state.device().create_command_encoder(&Default::default());
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some(&NeedleLabel::ImguiWindow("").to_string()),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        match self.renderer.render(
            self.context.render(),
            state.queue(),
            state.device(),
            &mut render_pass,
        ) {
            Ok(_) => Ok(()),
            Err(err) => Err(NeedleError::FailedToRenderUi(err.into())),
        }?;

        drop(render_pass);

        state.queue().submit(Some(encoder.finish()));

        Ok(())
    }

    pub fn handle_event(
        &mut self,
        window: &Window,
        window_id: WindowId,
        event: winit::event::WindowEvent,
    ) {
        self.platform.handle_event::<()>(
            self.context.io_mut(),
            window,
            &winit::event::Event::WindowEvent { event, window_id },
        )
    }

    #[inline]
    pub fn toggle_imgui(&mut self) {
        self.show_imgui = !self.show_imgui;
    }

    fn create_context(window: Arc<Window>, _config: Rc<RefCell<NeedleConfig>>) -> Context {
        let mut context = Context::create();
        let hidpi_factor = window.scale_factor();
        let font_size = (13.0 * hidpi_factor) as f32;

        context.set_ini_filename(None);
        context.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;
        context.fonts().add_font(&[FontSource::DefaultFontData {
            config: Some(FontConfig {
                oversample_h: 1,
                pixel_snap_h: true,
                size_pixels: font_size,
                ..Default::default()
            }),
        }]);

        context
    }

    fn create_platform(window: Arc<Window>, context: &mut Context) -> WinitPlatform {
        let mut platform = WinitPlatform::new(context);

        platform.attach_window(context.io_mut(), &window, HiDpiMode::Default);

        platform
    }

    fn create_renderer(context: &mut Context, state: &State) -> imgui_wgpu::Renderer {
        let config = imgui_wgpu::RendererConfig {
            texture_format: state.surface_config().format,
            ..Default::default()
        };

        imgui_wgpu::Renderer::new(context, state.device(), state.queue(), config)
    }
}
