mod cli;
mod config;
mod generator;
mod process;
mod template;
mod ui;

use clap::Parser;
use generator::project::generate_project;

use crate::{
    cli::{args::Cli, prompts::collect_project_config},
    ui::{
        banner::print_banner, last_message::print_last_message, notes::print_notes,
        seperater::print_separator,
    },
};

fn main() -> anyhow::Result<()> {
    print_banner();

    print_notes();

    print_separator(Some(true));

    let cli = Cli::parse();

    let config = collect_project_config(cli)?;

    print_separator(Some(true));

    generate_project(&config)?;

    print_separator(Some(true));

    print_last_message(&config);

    Ok(())
}
