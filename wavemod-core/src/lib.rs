#![allow(unused_variables)]

#[macro_use]
mod global;
// mod utils;
#[macro_use]
mod schema;
mod board;
mod commands;
mod graphics;
#[cfg(not(target_arch = "wasm32"))]
mod python;
mod setup;

pub mod utils;
pub use utils::*;

mod example;

mod renderer;
pub use renderer::*;

mod surface_wrapper;
pub use surface_wrapper::*;

mod iadq_context;
pub use iadq_context::*;

use crate::setup::setup_logging;

use std::env;

#[cfg(target_arch = "wasm32")]
use std::rc::Rc as SharedPtr;
#[cfg(not(target_arch = "wasm32"))]
use std::sync::Arc as SharedPtr;

pub fn run() {
	setup_logging();

	log::info!("Operating System: {}", env::consts::OS);
	log::info!("Architecture: {}", env::consts::ARCH);

	log::info!("Initializing application");

	let title = "wavemod-core";
	let loop_function = || setup::setup_app::<example::BunnyRenderer>(title.to_string());

	cfg_if::cfg_if! {
	  if #[cfg(target_arch = "wasm32")] {
	   wasm_bindgen_futures::spawn_local(async move { loop_function().await })
	  } else {
	   pollster::block_on(loop_function());
	  }
	}
}
