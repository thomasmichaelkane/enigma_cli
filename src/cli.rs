use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "enigma",
    about = "cli enigma machine",
    version,
    author
)]
pub struct Cli {
    /// Run in debug mode => Supress ASCII and print received signals and internal permutations.
    #[arg(short = 'd', long = "debug")]
    pub debug_mode: bool,

    /// Run in secret mode => The message will not be displayed to the screen.
    #[arg(short = 's', long = "secret")]
    pub secret_mode: bool,

    /// Show key bindings and instructions at the top of the screen.
    #[arg(short = 'i', long = "instruct")]
    pub show_instructions: bool,

    /// Run with animations => Activate animations.
    #[arg(short = 'a', long = "animate")]
    pub animate: bool,
}