// src/main.rs
/*
 * Main executable for StablecoinPeg
 * 
 * This file contains the main entry point for the StablecoinPeg CLI tool.
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

/// Run the StablecoinPeg executable with the provided arguments
fn run_stablecoinpeg(verbose: bool, input: Option<String>, output: Option<String>) -> Result<()> {
    // Run the StablecoinPeg executable with the provided arguments
    run(verbose, input, output)
}

fn main() -> Result<()> {
    // Parse command-line arguments
    let args = Cli::parse();
    
    // Run the StablecoinPeg executable with the provided arguments
    run_stablecoinpeg(args.verbose, args.input, args.output)
}