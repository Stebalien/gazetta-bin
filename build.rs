use std::{env, io};

use clap::{CommandFactory, ValueEnum};

fn main() -> Result<(), io::Error> {
    let Some(outdir) = env::var_os("OUT_DIR") else {
        return Ok(());
    };

    let mut cmd = gazetta::cli::Cli::command();
    let Some(name) = env::var_os("CARGO_BIN_NAME") else {
        // Not building the binary.
        return Ok(());
    };

    let name = name.to_string_lossy();
    for &shell in clap_complete::Shell::value_variants() {
        clap_complete::generate_to(shell, &mut cmd, &*name, &outdir)?;
    }
    Ok(())
}
