use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn surfboard(canvas_id: &str) {
    
    let title = "waveboard-canvas";
    let start_fn = || waveboard_wasm::start::<waveboard_wasm::waveboard::BunnyRenderer>(title); 

    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            wasm_bindgen_futures::spawn_local(async move { start_fn().await })
        } else {
            pollster::block_on(start_fn());
        }
    }

    log(format!("Targeting canvas: {}", canvas_id).as_str());
}

fn main() {
}

