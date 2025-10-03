use std::sync::Arc;

use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use unen_runner::prelude::{Runner, SharedRunnerData};

use unen_window::prelude::{SendableWindowHandle, WindowEvent};
use winit::{
    application::ApplicationHandler,
    event::KeyEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use winit::event_loop::EventLoopProxy;
#[cfg(target_arch = "wasm32")]
use winit::platform::web::WindowAttributesExtWebSys;

struct State {
    #[allow(dead_code)]
    window: Arc<Window>,
}

impl State {
    pub async fn new(window: Arc<Window>) -> Self {
        Self { window }
    }
}

#[derive(Default)]
pub struct WinitRunner {
    #[cfg(target_arch = "wasm32")]
    proxy: Option<EventLoopProxy<State>>,
    state: Option<State>,
    runner_data: Option<SharedRunnerData>,
}

impl Runner for WinitRunner {
    fn run(&mut self, data: SharedRunnerData) {
        let event_loop = EventLoop::with_user_event().build().unwrap();
        self.runner_data = Some(data);
        event_loop.run_app(self).unwrap();
    }
}

impl WinitRunner {
    pub fn new(#[cfg(target_arch = "wasm32")] event_loop: &EventLoop<State>) -> Self {
        #[cfg(target_arch = "wasm32")]
        let proxy = Some(event_loop.create_proxy());

        Self {
            #[cfg(target_arch = "wasm32")]
            proxy,
            state: None,
            runner_data: None,
        }
    }
}

impl ApplicationHandler<State> for WinitRunner {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        #[allow(unused_mut)]
        let mut window_attributes = Window::default_attributes();

        #[cfg(target_arch = "wasm32")]
        {
            const CANVAS_ID: &str = "canvas";
            let window = wgpu::web_sys::window().unwrap_throw();
            let document = window.document().unwrap_throw();
            let canvas = document.get_element_by_id(CANVAS_ID).unwrap_throw();
            let html_canvas_element = canvas.unchecked_into();
            window_attributes = window_attributes.with_canvas(Some(html_canvas_element));
        }

        let runner_data = match &mut self.runner_data {
            Some(data) => data,
            None => return,
        };

        let runner_data = match runner_data.lock() {
            Ok(data) => data,
            Err(_) => return,
        };

        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
        let raw_window_handle = window.window_handle().unwrap().as_raw();
        let raw_display_handle = window.display_handle().unwrap().as_raw();
        let sendable_window_handle =
            SendableWindowHandle::new(raw_window_handle, raw_display_handle);
        runner_data
            .event_emitter
            .emit(WindowEvent::Created(sendable_window_handle));

        #[cfg(not(target_arch = "wasm32"))]
        {
            // If we are not on web we can use pollster to await the state
            self.state = Some(pollster::block_on(State::new(window)));
        }

        #[cfg(target_arch = "wasm32")]
        {
            // Run the future asynchronously and use the proxy to send the
            // results to the event loop
            if let Some(proxy) = self.proxy.take() {
                wasm_bindgen_futures::spawn_local(async move {
                    assert!(proxy.send_event(State::new(window).await)).is_ok()
                });
            }
        }
    }

    #[allow(unused_mut)]
    fn user_event(&mut self, _event_loop: &ActiveEventLoop, mut event: State) {
        // This is where proxy.send_event() ends up
        #[cfg(target_arch = "wasm32")]
        {
            event.window.request_redraw();
            event.resize(
                event.window.inner_size().width,
                event.window.inner_size().height,
            );
        }
        self.state = Some(event);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let runner_data = match &mut self.runner_data {
            Some(data) => data,
            None => return,
        };

        let mut runner_data = match runner_data.lock() {
            Ok(data) => data,
            Err(_) => return,
        };

        match event {
            winit::event::WindowEvent::CloseRequested => {
                runner_data.event_emitter.emit(WindowEvent::Destroyed);
                event_loop.exit();
            }
            winit::event::WindowEvent::Resized(size) => {
                runner_data.event_emitter.emit(WindowEvent::Resized {
                    width: size.width,
                    height: size.height,
                });
            }
            winit::event::WindowEvent::RedrawRequested => {
                runner_data.event_emitter.emit(WindowEvent::Redraw);
                runner_data.event_manager.step();
            }
            winit::event::WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state: key_state,
                        ..
                    },
                ..
            } => {
                if let (KeyCode::Escape, true) = (code, key_state.is_pressed()) {
                    event_loop.exit()
                }
            }
            _ => {}
        }
    }
}
