use needle_core::{Renderer, State, TextRenderer, TextRendererDescriptor};
use std::sync::Arc;
use winit::{
    event::{ElementState, KeyEvent, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

#[derive(Default)]
struct App<'a> {
    renderer: Option<TextRenderer>,
    state: Option<State<'a>>,
    window: Option<Arc<Window>>,
}

impl App<'_> {
    fn handle_keyevent(event: &KeyEvent, event_loop: &winit::event_loop::ActiveEventLoop) {
        match event.physical_key {
            PhysicalKey::Code(KeyCode::Escape) | PhysicalKey::Code(KeyCode::KeyQ)
                if event.state == ElementState::Pressed =>
            {
                event_loop.exit();
            }
            _ => (),
        }
    }

    fn resize(&mut self, size: &winit::dpi::PhysicalSize<u32>) {
        #[cfg(debug_assertions)]
        assert!(self.window.is_some() && self.state.is_some());

        if let Some(window) = self.window.clone()
            && let Some(state) = self.state.as_mut()
        {
            state.resize(size);
            window.request_redraw();
        }
    }

    fn redraw(&mut self) -> anyhow::Result<()> {
        #[cfg(debug_assertions)]
        assert!(self.window.is_some() && self.state.is_some() && self.renderer.is_some());

        if let Some(window) = self.window.clone()
            && let Some(state) = self.state.as_mut()
            && let Some(renderer) = self.renderer.as_mut()
        {
            state.device().poll(wgpu::PollType::Wait {
                submission_index: None,
                timeout: None,
            })?;

            // Render logic
            renderer.update(state)?;
            let texture = match state.get_current_texture(window.clone()) {
                Ok(texture) => texture,
                Err(err) => {
                    eprintln!("{err}");

                    return Ok(());
                }
            };
            let view = texture
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());
            window.pre_present_notify();
            if let Err(err) = state.render(|encoder| {
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        depth_slice: None,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                    multiview_mask: None,
                });

                renderer.render(&mut pass)?;

                drop(pass);

                Ok(())
            }) {
                panic!("{err}")
            }
            state.queue().present(texture);
            renderer.trim();
        }

        Ok(())
    }
}

impl winit::application::ApplicationHandler for App<'_> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.window.is_none() {
            match event_loop
                .create_window(Window::default_attributes().with_title("Text Renderer example"))
            {
                Ok(window) => self.window = Some(Arc::new(window)),
                Err(err) => panic!("{err}"),
            };
        }

        if self.state.is_none()
            && let Some(window) = self.window.clone()
        {
            match pollster::block_on(State::new(window)) {
                Ok(state) => self.state = Some(state),
                Err(err) => panic!("{err}"),
            };
        }

        if self.renderer.is_none()
            && let Some(window) = self.window.clone()
            && let Some(state) = self.state.as_ref()
        {
            match TextRenderer::new(
                state,
                &TextRendererDescriptor {
                    config: &needle_core::Text {
                        scale: 0.5f32,
                        color: [255u8; 4],
                        position: needle_core::Position::Center,
                    },
                    font: None,
                    font_size: window.inner_size(),
                    scale_factor: 1f32,
                    margin: 1f32,
                    format: state.surface_config().format,
                    depth_stencil: None,
                },
            ) {
                Ok(mut renderer) => {
                    renderer.set_text("Hello World");
                    renderer.resize(&window.inner_size());

                    self.renderer = Some(renderer);
                }
                Err(err) => panic!("{err}"),
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _: winit::window::WindowId,
        window_event: WindowEvent,
    ) {
        if self.window.is_none() || self.state.is_none() || self.renderer.is_none() {
            return;
        }

        match window_event {
            WindowEvent::Resized(size) => self.resize(&size),
            WindowEvent::RedrawRequested => {
                if let Err(err) = self.redraw() {
                    panic!("{err}")
                }
            }
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::KeyboardInput { event, .. } => Self::handle_keyevent(&event, event_loop),
            _ => (),
        }
    }
}

fn main() -> anyhow::Result<()> {
    let event_loop = winit::event_loop::EventLoop::new()?;
    let mut app = App::default();

    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    event_loop.run_app(&mut app)?;

    Ok(())
}
