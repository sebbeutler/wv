#![allow(unused_variables)]

#[macro_use]
mod global;
// mod utils;
#[macro_use]
mod schema;
mod board;
mod commands;
mod graphics;
mod python;
mod setup;
// #[cfg(all(desktop, not(debug_assertions)))]
// mod update;

use crate::setup::{setup_app, setup_logging};

#[macro_use]
extern crate tracing;

use std::env;

pub fn run() {
    setup_logging();

    info!("Operating System: {}", env::consts::OS);
    info!("Architecture: {}", env::consts::ARCH);

    info!("Initializing application");

    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .setup(setup_app)
        .invoke_handler(tauri::generate_handler![
            commands::quit,
            commands::get_board_props,
            // commands::draw_shader,
            commands::trace,
            commands::debug,
            commands::info,
            commands::warn,
            commands::error,
            commands::devtools,
            commands::show_window,
            commands::gx_init,
        ])
        .run(tauri::generate_context!())
        .expect("error while building tauri application");
    // .run(|app_handle, event| {
    //     if let tauri::RunEvent::Ready = event {
    //         println!("App ready");
    //     } else if let tauri::RunEvent::WindowEvent { label, event, .. } = event {
    //         println!("@Window: {} {:?}", label, event);
    //     } else if let tauri::RunEvent::WebviewEvent { label, event, .. } = event {
    //         println!("@Webview: {} {:?}", label, event);
    //     }
    // });
}
