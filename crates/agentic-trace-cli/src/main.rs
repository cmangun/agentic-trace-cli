use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "agentic-trace", about = "CLI for verifiable agent traces", version)]
struct Cli { #[command(subcommand)] command: Commands }

#[derive(Subcommand)]
enum Commands {
    Init { path: PathBuf },
    Append { path: PathBuf, #[arg(long)] event: PathBuf },
    Sign { path: PathBuf, #[arg(long)] key: PathBuf },
    Verify { path: PathBuf },
    Redact { path: PathBuf },
    Export { path: PathBuf, #[arg(long, default_value = "json")] format: String },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Init { path } => { agentic_trace::Bundle::init(path).expect("Failed to init"); }
        _ => { todo!("Implement command") }
    }
}
