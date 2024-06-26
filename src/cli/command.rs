// SPDX-FileCopyrightText: 2023-2024 Christina Sørensen
// SPDX-FileContributor: Christina Sørensen
//
// SPDX-License-Identifier: EUPL-1.2

use clap::{arg, command, crate_authors, Command};

/// Parses command-line arguments using the `clap` library.
///
/// # Returns
///
/// Returns an instance of `ArgMatches` which contains the parsed arguments.
///
pub fn build_cli() -> Command {
    command!()
        .author(crate_authors!("\n"))
        .arg(arg!(--init <path> "Init config.yaml"))
        .arg(arg!(-c --config [config] "Specify config file"))
        .arg(arg!(-a --addr [addr] "Specify addr to launch server on"))
        .arg(arg!(-p --port [port] "Specify port to launch server on"))
}
