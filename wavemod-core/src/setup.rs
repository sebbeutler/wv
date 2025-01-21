#![allow(unused)]
#![allow(clippy::arc_with_non_send_sync)] // False positive on wasm
#![warn(clippy::allow_attributes)]

use winit::{
	event::{Event, KeyEvent, WindowEvent},
	event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
	keyboard::{Key, NamedKey},
	window::Window,
};

use crate::{Renderer, SharedPtr};

pub fn setup_logging() {
	cfg_if::cfg_if! {
		if #[cfg(target_arch = "wasm32")] {
			// As we don't have an environment to pull logging level from, we use the query string.
			let query_string = web_sys::window().unwrap().location().search().unwrap();
			let query_level: Option<log::LevelFilter> = crate::utils::parse_url_query_string(&query_string, "RUST_LOG")
				.and_then(|x| x.parse().ok());

			// We keep wgpu at Error level, as it's very noisy.
			let base_level = query_level.unwrap_or(log::LevelFilter::Info);
			let wgpu_level = query_level.unwrap_or(log::LevelFilter::Error);

			// On web, we use fern, as console_log doesn't have filtering on a per-module level.
			fern::Dispatch::new()
				.level(base_level)
				.level_for("wgpu_core", wgpu_level)
				.level_for("wgpu_hal", wgpu_level)
				.level_for("naga", wgpu_level)
				.chain(fern::Output::call(console_log::log))
				.apply()
				.unwrap();
			std::panic::set_hook(Box::new(console_error_panic_hook::hook));
		} else {
			// parse_default_env will read the RUST_LOG environment variable and apply it on top
			// of these default filters.
			env_logger::builder()
				.filter_level(log::LevelFilter::Info)
				// We keep wgpu at Error level, as it's very noisy.
				.filter_module("wgpu_core", log::LevelFilter::Info)
				.filter_module("wgpu_hal", log::LevelFilter::Error)
				.filter_module("naga", log::LevelFilter::Error)
				.parse_default_env()
				.init();
		}
	}
}

pub(crate) async fn setup_app<R: Renderer>(
	title: String,
) -> Result<(), Box<dyn std::error::Error>> {
	println!("SETUP");
	log::debug!(
		"Enabled backends: {:?}",
		wgpu::Instance::enabled_backend_features()
	);

	/* BUILDING WINDOW */
	let event_loop = EventLoop::new().unwrap();
	let mut builder = winit::window::WindowBuilder::new().with_title(title);
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
	#[cfg(target_os = "windows")]
	{
		use winit::platform::windows::WindowBuilderExtWindows;
		builder = builder.with_undecorated_shadow(false);
	}
	let window = SharedPtr::new(builder.build(&event_loop).unwrap());
	#[cfg(target_os = "windows")]
	{
		use winit::platform::windows::WindowExtWindows;
		window.set_undecorated_shadow(true);
		// apply_window_effect(&window)?;
	}

	/* BUILDING WEBVIEW */
	#[cfg(not(target_arch = "wasm32"))]
	let _webview = {
		let builder = wry::WebViewBuilder::new()
			// .with_transparent(true)
			.with_devtools(true)
			.with_url("http://localhost:1420/");
		#[cfg(any(
			target_os = "windows",
			target_os = "macos",
			target_os = "ios",
			target_os = "android"
		))]
		let _webview = builder.build(&window)?;
		#[cfg(not(any(
			target_os = "windows",
			target_os = "macos",
			target_os = "ios",
			target_os = "android",
			target_arch = "wasm32"
		)))]
		let _webview = {
			use winit::platform::unix::WindowExtUnix;
			use wry::WebViewBuilderExtUnix;
			let vbox = window.default_vbox().unwrap();
			builder.build_gtk(vbox)?
		};
		_webview
	};
	println!("PREP LOOP");

	setup_eventloop::<R>(event_loop, window).await;
	Ok(())
}

async fn setup_eventloop<R: crate::Renderer>(event_loop: EventLoop<()>, window: SharedPtr<Window>) {
	println!("SETUPLOOP");

	let mut surface = crate::SurfaceWrapper::new();
	let context = crate::IADQContext::init_async::<R>(&mut surface, window.clone()).await;

	#[cfg(target_arch = "wasm32")]
	let mut frame_counter = FrameCounter::new();

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
	let _ = (event_loop_function)(
		event_loop,
		move |event: Event<()>, target: &EventLoopWindowTarget<()>| {
			match event {
				ref e if crate::SurfaceWrapper::start_condition(e) => {
					surface.resume(&context, window.clone(), R::SRGB);

					// If we haven't created the example yet, do so now.
					if example.is_none() {
						example = Some(R::init(
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
						event: KeyEvent {
							logical_key: Key::Character(s),
							..
						},
						..
					} if s == "r" => {
						println!("{:#?}", context.instance.generate_report());
					}

					WindowEvent::RedrawRequested => {
						if example.is_none() {
							return;
						}

						#[cfg(target_arch = "wasm32")]
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
	log::info!("...Finished event loop.");
}

#[cfg(any(target_os = "macos", target_os = "windows"))]
fn apply_window_effect(window: &winit::window::Window) -> Result<(), Box<dyn std::error::Error>> {
	#[cfg(target_os = "macos")]
	{
		log::info!("Applying vibrancy effect on macOS");
		use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};
		// window.set_title_bar_style(tauri::utils::TitleBarStyle::Overlay)?;
		apply_vibrancy(window, NSVisualEffectMaterial::HudWindow, None, None)?;
	}

	#[cfg(target_os = "windows")]
	{
		info!("Applying acrylic effect on Windows");
		use window_vibrancy::apply_acrylic;
		apply_acrylic(&window, Some((18, 18, 18, 125)))?;
	}

	Ok(())
}

#[cfg(target_arch = "wasm32")]
struct FrameCounter {
	// Instant of the last time we printed the frame time.
	last_printed_instant: web_time::Instant,
	// Number of frames since the last time we printed the frame time.
	frame_count: u32,
}
#[cfg(target_arch = "wasm32")]
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
