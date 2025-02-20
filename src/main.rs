use clap::Parser;

use enigma::cli::Cli;
use enigma::config::Config;
use enigma::model::EnigmaModel;
use enigma::view::EnigmaView;
use enigma::rotor::Rotor;
use enigma::plugboard::Plugboard;

use enigma::utils::{
    load_constants, 
    load_ascii_art, 
    load_ascii_mapping_top, 
    load_ascii_mapping_plugboard
};

fn main() {

    // Parse args
    let cli = Cli::parse();
    let config = Config::new(&cli);

    // Load ASCII art
    let frame = load_ascii_art()
        .expect("Failed to load ASCII art");

    // Load constants
    let constants = load_constants()
        .expect("Failed to load YAML");
    
    // Load ASCII character mapping for the top view
    let ascii_mapping_top = load_ascii_mapping_top();

    // Load ASCII character mapping for the plugboard view
    let ascii_mapping_plugboard = load_ascii_mapping_plugboard();

    // Create rotors
    let rotors: Vec<Rotor> = constants["rotor_permutations"]
        .as_vec()
        .expect("Expected rotor_permutations to be an array")
        .iter()
        .map(|perm| Rotor::new(perm
            .as_str()
            .expect("Expected string")
            .to_string()))
        .collect();

    // Create reflector
    let reflector: Option<Rotor> = Some(Rotor::new(constants["reflector_permutations"]
        .as_str()
        .expect("Expected string").to_string()));

    // Create plugboard
    let plugboard: Plugboard = Plugboard::new();
    
    // Create view
    let view = EnigmaView::new(
        frame,
        ascii_mapping_top,
        ascii_mapping_plugboard,
    );
    
    // Create model
    let mut enigma = EnigmaModel::new(
        view, 
        rotors, 
        reflector,
        plugboard,
        config); 

    // Wire plugboard
    enigma.wire_plugboard();

    // Run the enigma machine
    enigma.start_typing();

}