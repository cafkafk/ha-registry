// SPDX-FileCopyrightText: 2023-2024 Christina Sørensen
// SPDX-FileContributor: Christina Sørensen
//
// SPDX-License-Identifier: EUPL-1.2

use clap::ValueEnum;
use clap_complete::{generate_to, Shell};
use clap_mangen::Man;
use std::env;
use std::fs::File;
use std::io::Error;
use std::path::PathBuf;

include!("src/cli/command.rs");

fn main() -> Result<(), Error> {
    let real_outdir = match env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(outdir) => outdir,
    };

    let outdir = match env::var_os("MAN_OUT") {
        None => real_outdir,
        Some(outdir) => outdir,
    };

    let mut cmd = build_cli();
    for &shell in Shell::value_variants() {
        generate_to(shell, &mut cmd, "ha-registry", &outdir)?;
    }

    let file = PathBuf::from(&outdir).join("ha-registry.1");
    let mut file = File::create(file)?;

    Man::new(cmd).render(&mut file)?;

    println!("cargo:warning=completion file is generated: {outdir:?}");

    Ok(())
}
