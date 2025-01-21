#[tauri::command]
pub fn quit(app: tauri::AppHandle) {
    println!("Global Quit Requested");
    app.exit(0);
}

#[tauri::command]
pub fn get_board_props(
    boardstate: tauri::State<'_, crate::board::BoardStateMutex>,
) -> Result<crate::schema::SchemaObject, ()> {
    let schema = boardstate.lock().unwrap().properties().clone();
    Ok(schema)
}

use tauri::{Emitter, Manager};
use std::sync::Mutex;
#[tauri::command]
pub fn gx_init(app_handle: tauri::AppHandle, window: tauri::WebviewWindow) {
    if app_handle.try_state::<crate::graphics::GraphicsHandleMutex>().is_none() {
        let gx = crate::graphics::create_graphics(window.clone())
            .with_shader(include_str!("../assets/cosmos.wgsl")).apply_shader(0).spawn_render_loop();
        // window.manage(Mutex::new(gx.surface));
        // window.manage(Mutex::new(gx.device));
        // window.manage(Mutex::new(gx.queue));
        info!("WGPU renderer attached.");
    } else {
        info!("Existing GX; skipping creation.")
    }
}

#[tauri::command]
pub fn create_board(window: tauri::WebviewWindow) {
    window.manage(
        crate::board::create_board().as_mutex()
    );
}

#[tauri::command]
pub fn create_node(window: tauri::WebviewWindow, name: String) {
    let boardstate: tauri::State<'_, crate::board::BoardStateMutex> = window.state();
    let mut board = boardstate.lock().unwrap();
    board.add_node(
        crate::board::Node::new(name.as_str())
    );
}

// #[tauri::command]
// pub fn draw_shader(
//     graphics: tauri::State<'_, crate::graphics::GraphicsHandleMutex>,
//     shader_id: usize,
// ) {
//     let graphics = graphics.lock().expect("Graphics draw lock failed somehow?");
//     graphics.render_shader(shader_id);
// }

#[tauri::command]
pub async fn trace(prefix: String, msg: String) {
    trace!("{}: {}", prefix, msg);
}

#[tauri::command]
pub async fn debug(prefix: String, msg: String) {
    debug!("{}: {}", prefix, msg);
}

#[tauri::command]
pub async fn info(prefix: String, msg: String) {
    info!("{}: {}", prefix, msg);
}

#[tauri::command]
pub async fn warn(prefix: String, msg: String) {
    warn!("{}: {}", prefix, msg);
}

#[tauri::command]
pub async fn error(prefix: String, msg: String) {
    error!("{}: {}", prefix, msg);
}

#[cfg(debug_assertions)]
#[tauri::command]
pub async fn devtools(window: tauri::WebviewWindow) {
    window.open_devtools();
}

#[cfg(not(debug_assertions))]
#[tauri::command]
pub async fn devtools(window: tauri::WebviewWindow) {
    warn!("devtools unavi.");
}

#[tauri::command]
pub async fn show_window(window: tauri::WebviewWindow) {
    info!("Showing window");
    window.show().unwrap();
    window.set_focus().unwrap();
}

