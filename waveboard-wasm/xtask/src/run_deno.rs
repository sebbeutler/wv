use anyhow::Context;

use pico_args::Arguments;
use xshell::Shell;

use crate::util::{check_all_programs, Program};

pub(crate) fn run_deno(shell: &Shell, mut args: Arguments) -> anyhow::Result<()> {
    crate::build_wasm::build_wasm(shell, args)?;

    shell.change_dir("/Users/sb/Source/wv/waveboard");
    xshell::cmd!(shell, "deno task dev")
        .quiet()
        .run()
        .context("Failed to run deno task dev")?;

    Ok(())
}
