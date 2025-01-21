#![allow(unused_macros)]
#![allow(unused_imports)]

use md5::{Digest, Md5};

#[tauri::command]
pub async fn md5(text: String) -> String {
    let mut hasher = Md5::new();
    hasher.update(&text);
    let result = hasher.finalize();
    // let bytes: &[u8] = &result[..];
    // debug!("md5 bytes: {:?}", bytes);
    //     bytes
    //         .iter()
    //         .map(|b| format!("{:02x}", b).to_string())
    //         .collect::<String>()
    format!("{:x}", result)
}

