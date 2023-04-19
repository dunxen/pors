mod args;
mod errors;
mod commands;
mod output;

use args::{CliArgs, OutputFormat};
use clap::Parser;
use commands::handle_command;
use miette::Report;

fn main() {
    let config = CliArgs::parse();

    axocli::CliAppBuilder::new("pors")
        .verbose(config.verbose)
        .json_errors(config.output_format == OutputFormat::Json)
        .start(config, run);
}

/// Only call after everything has been set up properly
fn run(app: &axocli::CliApp<CliArgs>) -> Result<(), Report> {
    handle_command(&app.config.command)
}
