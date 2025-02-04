use std::process::ExitCode;

use anyhow::Context;
use pico_args::Arguments;

mod run_static;
mod test;
mod util;
mod vendor_web_sys;
mod wasm;

const HELP: &str = "\
Usage: xtask <COMMAND>

Commands:
  run-wasm
    Build and run web examples

    --release   Build in release mode
    --no-serve  Just build the generated files, don't serve them
";

/// Helper macro for printing the help message, then bailing with an error message.
#[macro_export]
macro_rules! bad_arguments {
    ($($arg:tt)*) => {{
        eprintln!("{}", crate::HELP);
        anyhow::bail!($($arg)*)
    }};
}

fn main() -> anyhow::Result<ExitCode> {
	env_logger::builder()
		.filter_level(log::LevelFilter::Info)
		.parse_default_env()
		.format_indent(Some(0))
		.init();

	let mut args = Arguments::from_env();

	if args.contains("--help") {
		eprint!("{HELP}");
		return Ok(ExitCode::FAILURE);
	}

	let subcommand = args
		.subcommand()
		.context("Expected subcommand to be UTF-8")?;

	// -- Shell Creation --

	let shell = xshell::Shell::new().context("Couldn't create xshell shell")?;
	shell.change_dir(String::from(env!("CARGO_MANIFEST_DIR")) + "/..");

	eprint!("TESR{}", String::from(env!("CARGO_MANIFEST_DIR")) + "/..");

	match subcommand.as_deref() {
		Some("build-wasm") => wasm::build_wasm(&shell, args)?,
		Some("run-wasm") => wasm::build_wasm(&shell, args)?,
		Some("clean-wasm") => wasm::build_wasm(&shell, args)?,
		Some("run-static") => run_static::run_static(&shell, args)?,
		Some("test") => test::run_tests(&shell, args)?,
		Some("vendor-web-sys") => vendor_web_sys::run_vendor_web_sys(&shell, args)?,
		Some(subcommand) => {
			bad_arguments!("Unknown subcommand: {}", subcommand)
		}
		None => {
			bad_arguments!("Expected subcommand")
		}
	}

	Ok(ExitCode::SUCCESS)
}
