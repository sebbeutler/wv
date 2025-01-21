use crate::SharedPtr;

pub fn quit() {
	println!("Global Quit Requested");
	// app.exit(0);
}

pub fn get_board_props(
	boardstate: crate::board::BoardStateMutex,
) -> Result<crate::schema::SchemaObject, ()> {
	let schema = boardstate.lock().unwrap().properties().clone();
	Ok(schema)
}

pub fn gx_init(window: SharedPtr<winit::window::Window>) {
	let gx = crate::graphics::create_graphics(window.clone())
		.with_shader(include_str!("./example/cosmos.wgsl"))
		.apply_shader(0)
		.spawn_render_loop();
	// window.manage(Mutex::new(gx.surface));
	// window.manage(Mutex::new(gx.device));
	// window.manage(Mutex::new(gx.queue));
	log::info!("WGPU renderer attached.");
}

pub fn create_board() {
	// manage(crate::board::create_board().as_mutex());
	log::error!("create_board not implemented!")
}

pub fn create_node(boardstate: crate::board::BoardStateMutex, name: String) {
	let mut board = boardstate.lock().unwrap();
	board.add_node(crate::board::Node::new(name.as_str()));
}

//
// pub fn draw_shader(
//     graphics: tauri::State<'_, crate::graphics::GraphicsHandleMutex>,
//     shader_id: usize,
// ) {
//     let graphics = graphics.lock().expect("Graphics draw lock failed somehow?");
//     graphics.render_shader(shader_id);
// }

pub async fn trace(prefix: String, msg: String) {
	log::trace!("{}: {}", prefix, msg);
}

pub async fn debug(prefix: String, msg: String) {
	log::debug!("{}: {}", prefix, msg);
}

pub async fn info(prefix: String, msg: String) {
	log::info!("{}: {}", prefix, msg);
}

pub async fn warn(prefix: String, msg: String) {
	log::warn!("{}: {}", prefix, msg);
}

pub async fn error(prefix: String, msg: String) {
	log::error!("{}: {}", prefix, msg);
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn devtools(window: wry::WebView) {
	window.open_devtools();
}

pub async fn show_window(window: winit::window::Window) {
	log::info!("Showing window");
	window.focus_window();
}
