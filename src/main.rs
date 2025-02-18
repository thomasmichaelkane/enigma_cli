mod model;
mod view;
mod utils;
mod rotor;
// mod plugboard;

use std::env;
use utils::{load_constants, load_ascii_art, load_ascii_mapping};
use view::EnigmaView;
use model::EnigmaModel;
use rotor::Rotor;
// use plugboard::Plugboard;

fn main() {

    // Parse args
    let args: Vec<String> = env::args().collect();
    let debug_mode = args.contains(&"-d".to_string());

    // Load ASCII art
    let frame = load_ascii_art()
        .expect("Failed to load ASCII art");
    
    // Load ASCII mapping
    let ascii_mapping = load_ascii_mapping();

    // Load constants
    let constants = load_constants()
        .expect("Failed to load YAML");

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
    
    // Create view
    let view = EnigmaView::new(frame, ascii_mapping, debug_mode);
    
    // Create model
    let mut enigma = EnigmaModel::new(view, rotors, reflector, debug_mode); 

    // Run the enigma machine
    enigma.run();

}