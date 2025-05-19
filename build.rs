use std::{env, io};

use clap::{CommandFactory, ValueEnum};

fn main() -> Result<(), io::Error> {
    let outdir = match env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(outdir) => outdir,
    };

    let mut cmd = gazetta::cli::Cli::command();
    let name = cmd.get_name().to_string();

    for &shell in clap_complete::Shell::value_variants() {
        clap_complete::generate_to(shell, &mut cmd, &name, &outdir)?;
    }
    Ok(())
}
