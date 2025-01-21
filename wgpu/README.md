watchexec -r -e rs,wasm,js,html "cargo xtask build-wasm"
cargo watch -s "cargo xtask build-wasm"
