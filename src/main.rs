use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Create,
    Delete,
    Edit,
    List,
    Restart,
    Start,
    Stop,
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        _ => {
            todo!();
        }
    }
}
