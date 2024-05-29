use clap::Parser;

mod cli;

fn main() {
    let cli = cli::CliArgs::parse();
    println!("{cli:#?}");
}
