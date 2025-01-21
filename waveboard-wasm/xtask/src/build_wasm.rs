use anyhow::Context;

use pico_args::Arguments;
use xshell::Shell;

use crate::util::{check_all_programs, Program};

pub(crate) fn build_wasm(shell: &Shell, mut args: Arguments) -> anyhow::Result<()> {
    let release = args.contains("--release");
        
    check_all_programs(&[Program {
        crate_name: "wasm-bindgen-cli",
        binary_name: "wasm-bindgen",
    }])?;
    let output_dir = "/Users/sb/Source/wv/waveboard/src/Components/webgl2/";
    let release_flag: &[_] = if release { &["--release"] } else { &[] };
    let target = if release { "release" } else { "debug" };

    log::info!("building webgpu examples");

    let cargo_args = args.finish();

    xshell::cmd!(
        shell,
        "cargo build --target wasm32-unknown-unknown --bin waveboard-wasm --no-default-features --features webgpu {release_flag...}"
    )
    .args(&cargo_args)
    .quiet()
    .run()
    .context("Failed to build webgpu examples for wasm")?;

    log::info!("running wasm-bindgen on webgpu examples");

    xshell::cmd!(
        shell,
        "wasm-bindgen target/wasm32-unknown-unknown/{target}/waveboard-wasm.wasm --target web --typescript --out-dir {output_dir} --out-name webgpu"
    )
    .quiet()
    .run()
    .context("Failed to run wasm-bindgen")?;

    log::info!("building webgl examples");

    xshell::cmd!(
        shell,
        "cargo build --target wasm32-unknown-unknown --bin waveboard-wasm --no-default-features --features webgl {release_flag...}"
    )
    .args(&cargo_args)
    .quiet()
    .run()
    .context("Failed to build webgl examples for wasm")?;

    log::info!("running wasm-bindgen on webgl examples");

    xshell::cmd!(
        shell,
        "wasm-bindgen target/wasm32-unknown-unknown/{target}/waveboard-wasm.wasm --target web --typescript --out-dir {output_dir} --out-name webgl2"
    )
    .quiet()
    .run()
    .context("Failed to run wasm-bindgen")?;

    Ok(())
}
