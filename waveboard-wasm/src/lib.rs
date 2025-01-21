#![allow(clippy::arc_with_non_send_sync)] // False positive on wasm
#![warn(clippy::allow_attributes)]

#[cfg(target_arch = "wasm32")]
use std::rc::Rc as SharedPtr;
#[cfg(not(target_arch = "wasm32"))]
use std::sync::Arc as SharedPtr;

pub mod utils;
pub use utils::*;

mod renderer;
pub use renderer::*;

mod surface_wrapper;
pub use surface_wrapper::*;

mod iadq_context;
pub use iadq_context::*;

// mod canvas;
// pub use canvas::*;

// mod offscreen_canvas;
// pub use offscreen_canvas::*;

pub mod waveboard;

use winit::{
    event::{Event, KeyEvent, WindowEvent},
    event_loop::{EventLoop, EventLoopWindowTarget},
    keyboard::{Key, NamedKey},
};


struct FrameCounter {
    // Instant of the last time we printed the frame time.
    last_printed_instant: web_time::Instant,
    // Number of frames since the last time we printed the frame time.
    frame_count: u32,
}

impl FrameCounter {
    fn new() -> Self {
        Self {
            last_printed_instant: web_time::Instant::now(),
            frame_count: 0,
        }
    }

    fn update(&mut self) {
        self.frame_count += 1;
        let new_instant = web_time::Instant::now();
        let elapsed_secs = (new_instant - self.last_printed_instant).as_secs_f32();
        if elapsed_secs > 1.0 {
            let elapsed_ms = elapsed_secs * 1000.0;
            let frame_time = elapsed_ms / self.frame_count as f32;
            let fps = self.frame_count as f32 / elapsed_secs;
            log::info!("Frame time {:.2}ms ({:.1} FPS)", frame_time, fps);

            self.last_printed_instant = new_instant;
            self.frame_count = 0;
        }
    }
}

pub async fn start<E: Renderer>(title: &str) {
    init_logger();

    log::debug!(
        "Enabled backends: {:?}",
        wgpu::Instance::enabled_backend_features()
    );

    let event_loop = EventLoop::new().unwrap();
    let mut builder = winit::window::WindowBuilder::new();
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsCast;
        use winit::platform::web::WindowBuilderExtWebSys;
        let canvas = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();
        builder = builder.with_canvas(Some(canvas));
    }
    builder = builder.with_title(title);
    let window = SharedPtr::new(builder.build(&event_loop).unwrap());

    let mut surface = SurfaceWrapper::new();
    let context = IADQContext::init_async::<E>(&mut surface, window.clone()).await;
    let mut frame_counter = FrameCounter::new();

    log::info!("Example: {}", title);
    // We wait to create the example until we have a valid surface.
    let mut example = None;

    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            use winit::platform::web::EventLoopExtWebSys;
            let event_loop_function = EventLoop::spawn;
        } else {
            let event_loop_function = EventLoop::run;
        }
    }

    log::info!("Entering event loop...");
    #[cfg_attr(target_arch = "wasm32", expect(clippy::let_unit_value))]
    let _ = (event_loop_function)(
        event_loop,
        move |event: Event<()>, target: &EventLoopWindowTarget<()>| {
            match event {
                ref e if SurfaceWrapper::start_condition(e) => {
                    surface.resume(&context, window.clone(), E::SRGB);

                    // If we haven't created the example yet, do so now.
                    if example.is_none() {
                        example = Some(E::init(
                            surface.config(),
                            &context.adapter,
                            &context.device,
                            &context.queue,
                        ));
                    }
                }
                Event::Suspended => {
                    surface.suspend();
                }
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::Resized(size) => {
                        surface.resize(&context, size);
                        example.as_mut().unwrap().resize(
                            surface.config(),
                            &context.device,
                            &context.queue,
                        );

                        window.request_redraw();
                    }
                    WindowEvent::KeyboardInput {
                        event:
                            KeyEvent {
                                logical_key: Key::Named(NamedKey::Escape),
                                ..
                            },
                        ..
                    }
                    | WindowEvent::CloseRequested => {
                        log::info!("CloseRequested");
                        example = None;
                        target.exit();
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    WindowEvent::KeyboardInput {
                        event:
                            KeyEvent {
                                logical_key: Key::Character(s),
                                ..
                            },
                        ..
                    } if s == "r" => {
                        println!("{:#?}", context.instance.generate_report());
                    }
                    WindowEvent::RedrawRequested => {
                        // On MacOS, currently redraw requested comes in _before_ Init does.
                        // If this happens, just drop the requested redraw on the floor.
                        //
                        // See https://github.com/rust-windowing/winit/issues/3235 for some discussion
                        if example.is_none() {
                            return;
                        }

                        frame_counter.update();

                        let frame = surface.acquire(&context);
                        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor {
                            format: Some(surface.config().view_formats[0]),
                            ..wgpu::TextureViewDescriptor::default()
                        });

                        example
                            .as_mut()
                            .unwrap()
                            .render(&view, &context.device, &context.queue);

                        frame.present();

                        window.request_redraw();
                    }
                    _ => example.as_mut().unwrap().update(event),
                },
                _ => {}
            }
        },
    );
}

