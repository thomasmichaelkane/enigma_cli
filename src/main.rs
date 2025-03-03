use std::process;
use clap::Parser;

use enigma::cli::Cli;
use enigma::config::Config;

fn main() {

    // Parse args
    let cli = Cli::parse();
    let config = Config::new(&cli);

    if let Err(err) = enigma::run(config) {
        eprintln!("Application error: {}", err);
        process::exit(1);
    }

}