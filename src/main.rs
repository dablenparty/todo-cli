use clap::Parser;

mod cli;

fn main() {
    let cli = cli::CliArgs::parse();
    println!("{cli:#?}");

    if let Some(cmd) = cli.subcommand {
        cmd.handle_command();
    }

    todo!("Handle interactive mode")
}
