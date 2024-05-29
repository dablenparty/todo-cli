use clap::Parser;
use serde::{Deserialize, Serialize};

mod cli;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
struct TodoItem {
    pub short_desc: String,
    pub long_desc: Option<String>,
    pub completed: bool,
    // TODO: due date?
}

fn main() {
    let cli = cli::CliArgs::parse();
    println!("{cli:#?}");

    if let Some(cmd) = cli.subcommand {
        cmd.handle_command();
    }

    todo!("Handle interactive mode")
}
