// src/main.rs
/*
 * Main executable for StablecoinPeg
 */

use clap::Parser;
use stablecoinpeg::{Result, run};

/// CLI argument parser for StablecoinPeg
#[derive(Parser)]
#[command(version, about = "StablecoinPeg - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
    
    /// Path to input file (default: read from standard input)
    #[arg(short, long, default_value = "-")]
    input: Option<String>,
    
    /// Path to output file (default: write to standard output)
    #[arg(short, long, default_value = "-")]
    output: Option<String>,
}

fn main() -> Result<()> {
    // Parse command-line arguments
    let args = Cli::parse();
    
    // Run the StablecoinPeg executable with the provided arguments
    run(args.verbose, args.input, args.output)
}