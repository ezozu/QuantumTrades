// src/main.rs
/*
 * Main executable for QuantumTrades
 */

use clap::Parser;
use quantumtrades::{Result, run};

#[derive(Parser)]
#[command(version, about = "QuantumTrades - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
